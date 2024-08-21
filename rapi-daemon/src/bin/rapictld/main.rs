mod args;
mod job;
mod strategy;

use args::Args;
use clap::Parser;
use job::{Job, ProcessStatus};
use log::*;
use rapi::net::Connection;
use rapi::req::{ReqType, Request};
use rapi::*;
use simplelog::{Config, SimpleLogger};
use std::io;
use std::mem::drop;
use std::sync::{mpsc, Arc, RwLock};
use std::thread::{self, sleep};
use std::time::Duration;
use strategy::Strategy;

const REQ_CONT: Request = Request {
    req_type: ReqType::Cont,
    pid: 0,
};
const REQ_STOP: Request = Request {
    req_type: ReqType::Stop,
    pid: 0,
};

fn main() {
    let args = Args::parse();
    SimpleLogger::init(args.debug, Config::default()).unwrap();
    debug!("{:?}", args);

    let mut connections: Vec<(usize, Connection)> = Vec::new();
    for (i, addr) in args.rapid_addrs.iter().enumerate() {
        let port = args.port + i as u16;
        let c = Connection::new((BIND_ADDR, port), addr, args.rapid_port).unwrap();
        info!("Connected rapid: '{}' port: {}", addr, port);
        connections.push((i, c));
    }

    let mut strategy: Box<dyn Strategy> = match args.strategy {
        args::Strategy::Fixed(args) => {
            let dur = Duration::from_millis(args.timeslice);
            info!("Use strategy: FixedTimeslice");
            Box::new(strategy::FixedTimeslice::new(dur))
        }
        args::Strategy::CommFocused(args) => {
            let ts_min = Duration::from_millis(args.timeslice_min);
            let ts_max = Duration::from_millis(args.timeslice_max);
            let sleep_time = Duration::from_millis(args.sleep_time);
            info!("Use strategy: CommFocused");
            Box::new(strategy::CommFocused::new(ts_min, ts_max, sleep_time))
        }
        args::Strategy::WaitFocused(args) => {
            let ts_min = Duration::from_millis(args.timeslice_min);
            let ts_max = Duration::from_millis(args.timeslice_max);
            let sleep_time = Duration::from_millis(args.sleep_time);
            info!("Use strategy: WaitFocused");
            Box::new(strategy::WaitFocused::new(ts_min, ts_max, sleep_time))
        }
    };

    let job = Arc::new(RwLock::new(Job::new()));
    let (sender, recver) = mpsc::channel::<()>();

    // Create threads to receive message
    for connection in connections.iter() {
        let job = job.clone();
        let dest_id = connection.0;
        let connection = connection.1.try_clone().unwrap();
        let sender = sender.clone();
        thread::spawn(move || treat_msg(connection, dest_id, job, sender));
    }

    let polling_interval = Duration::from_micros(args.polling_interval);

    // Block until job is initialized
    recver.recv().unwrap();
    drop(recver);
    strategy.job_starts();
    info!("Job starts");

    'job_loop: loop {
        loop {
            if !job.read().unwrap().is_running() {
                break 'job_loop;
            } else if strategy.should_stop_job(job.clone()) {
                debug!("Stop job");
                break;
            } else {
                trace!("Polling job stopping");
                sleep(polling_interval);
            }
        }
        send_req_to_all(&mut connections, REQ_STOP).unwrap();

        loop {
            if !job.read().unwrap().is_running() {
                break 'job_loop;
            } else if strategy.should_start_job(job.clone()) {
                debug!("Resume job");
                break;
            } else {
                trace!("Polling job starting");
                sleep(polling_interval);
            }
        }
        send_req_to_all(&mut connections, REQ_CONT).unwrap();
    }

    info!("Job ends");
}

fn send_req_to_all(connections: &mut Vec<(usize, Connection)>, req: Request) -> io::Result<()> {
    for connection in connections {
        connection.1.send_req(&req)?;
    }
    trace!("Send request to all rapid: {:?}", req);
    Ok(())
}

fn treat_msg(
    mut connection: Connection,
    dest_id: usize,
    job: Arc<RwLock<Job>>,
    sender: mpsc::Sender<()>,
) {
    debug!("Start thread");
    loop {
        let msg = connection.recv_req().unwrap();
        trace!("Recv request: {:?}", msg);
        match msg.req_type {
            ReqType::Initialize => {
                let mut job = job.write().unwrap();
                job.append(dest_id, msg.pid.try_into().unwrap());
                let _res = sender.send(());
            }
            ReqType::Finalize => {
                let mut job = job.write().unwrap();
                job.remove(dest_id, msg.pid.try_into().unwrap());
            }
            ReqType::CommBegin => {
                let mut job = job.write().unwrap();
                job.change_state(
                    dest_id,
                    msg.pid.try_into().unwrap(),
                    ProcessStatus::Communicating,
                );
            }
            ReqType::CommEnd => {
                let mut job = job.write().unwrap();
                job.change_state(
                    dest_id,
                    msg.pid.try_into().unwrap(),
                    ProcessStatus::Calculating,
                );
            }
            ReqType::WaitBegin => {
                let mut job = job.write().unwrap();
                job.change_state(dest_id, msg.pid.try_into().unwrap(), ProcessStatus::Waiting);
            }
            ReqType::WaitEnd => {
                let mut job = job.write().unwrap();
                job.change_state(
                    dest_id,
                    msg.pid.try_into().unwrap(),
                    ProcessStatus::Calculating,
                );
            }
            _ => {
                warn!("Unexpected Message: {:?}", msg);
            }
        };
    }
}
