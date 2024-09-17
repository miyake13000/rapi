#include "rapi.h"

#include <mpi.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
struct timespec start_ct, end_ct;
struct timespec start_rt, end_rt;
pid_t pid;
int is_initialized = 0;
int fd;

double nsec_to_sec(time_t nsec) { return (double)nsec / (1000 * 1000 * 1000); }

double timespec_to_sec(time_t sec, time_t nsec) {
    return (double)sec + nsec_to_sec(nsec);
}

double calc_elapsed_time(struct timespec start, struct timespec end) {
    return timespec_to_sec(end.tv_sec - start.tv_sec, end.tv_nsec - start.tv_nsec);
}


// Count up the number of reveived SIGCONT
volatile sig_atomic_t num_sigcont = 0;
void sigcont_handler(int signum) { num_sigcont += 1; }

int MPI_Init(int *argc, char ***argv) {
    int ret;

    // Insert handler for SIGCONT
    signal(SIGCONT, sigcont_handler);

    // Initialize variables
    pid = getpid();
    fd = create_udp_socket();
    is_initialized = 1;

    clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &start_ct);
    clock_gettime(CLOCK_REALTIME, &start_rt);

    ret = PMPI_Init(argc, argv);
    send_req_to_rapid(fd, htonl(INADDR_LOOPBACK), get_rapid_port(),
                            (struct Request){.t = REQ_INIT, .pid = pid});

    return ret;
}

int MPI_Init_thread(int *argc, char ***argv, int required, int *provided) {
    int ret;

    // Insert handler for SIGCONT
    signal(SIGCONT, sigcont_handler);

    // Initialize variables
    pid = getpid();
    fd = create_udp_socket();

    clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &start_ct);
    clock_gettime(CLOCK_REALTIME, &start_rt);

    ret = PMPI_Init_thread(argc, argv, required, provided);
    send_req_to_rapid(fd, htonl(INADDR_LOOPBACK), get_rapid_port(),
                            (struct Request){.t = REQ_INIT, .pid = pid});

    return ret;
}

int MPI_Finalize() {
    int ret;

    int rank;
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);

    ret = PMPI_Finalize();
    send_req_to_rapid(fd, htonl(INADDR_LOOPBACK), get_rapid_port(),
                            (struct Request){.t = REQ_FINAL, .pid = pid});

    clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &end_ct);
    clock_gettime(CLOCK_REALTIME, &end_rt);
    printf("%d, %f, %f, %d\n", rank, calc_elapsed_time(start_rt, end_rt),
           calc_elapsed_time(start_ct, end_ct), num_sigcont);

    return ret;
}

int MPI_Scatter(const void *sendbuf, int sendcount, MPI_Datatype sendtype, void *recvbuf, int recvcount, MPI_Datatype recvtype, int root, MPI_Comm comm) {
    int ret;

    if (is_initialized == 1) {
        send_req_to_rapid(fd, htonl(INADDR_LOOPBACK), get_rapid_port(),
                (struct Request){.t = REQ_COMM_BEGIN, .pid = pid});
    }
    ret = PMPI_Scatter(sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, root, comm);
    if (is_initialized == 1) {
        send_req_to_rapid(fd, htonl(INADDR_LOOPBACK), get_rapid_port(),
                (struct Request){.t = REQ_COMM_END, .pid = pid});
    }
    return ret;
}

int MPI_Send(const void *buf, int count, MPI_Datatype datatype, int dest, int tag,
             MPI_Comm comm) {
    int ret;

    if (is_initialized == 1) {
        send_req_to_rapid(fd, htonl(INADDR_LOOPBACK), get_rapid_port(),
                (struct Request){.t = REQ_COMM_BEGIN, .pid = pid});
    }
    ret = PMPI_Send(buf, count, datatype, dest, tag, comm);
    if (is_initialized == 1) {
        send_req_to_rapid(fd, htonl(INADDR_LOOPBACK), get_rapid_port(),
                (struct Request){.t = REQ_COMM_END, .pid = pid});
    }

    return ret;
}

int MPI_Recv(void *buf, int count, MPI_Datatype datatype, int source, int tag,
             MPI_Comm comm, MPI_Status *status) {
    int ret;

    if (is_initialized == 1) {
        send_req_to_rapid(fd, htonl(INADDR_LOOPBACK), get_rapid_port(),
                (struct Request){.t = REQ_COMM_BEGIN, .pid = pid});
    }
    ret = PMPI_Recv(buf, count, datatype, source, tag, comm, status);
    if (is_initialized == 1) {
        send_req_to_rapid(fd, htonl(INADDR_LOOPBACK), get_rapid_port(),
                (struct Request){.t = REQ_COMM_END, .pid = pid});
    }

    return ret;
}

int MPI_Sendrecv(const void *sendbuf, int sendcount, MPI_Datatype sendtype, int dest,
                 int sendtag, void *recvbuf, int recvcount, MPI_Datatype recvtype,
                 int source, int recvtag, MPI_Comm comm, MPI_Status *status) {
    int ret;

    if (is_initialized == 1) {
        send_req_to_rapid(fd, htonl(INADDR_LOOPBACK), get_rapid_port(),
                (struct Request){.t = REQ_COMM_BEGIN, .pid = pid});
    }
    ret = PMPI_Sendrecv(sendbuf, sendcount, sendtype, dest, sendtag, recvbuf, recvcount,
            recvtype, source, recvtag, comm, status);
    if (is_initialized == 1) {
        send_req_to_rapid(fd, htonl(INADDR_LOOPBACK), get_rapid_port(),
                (struct Request){.t = REQ_COMM_END, .pid = pid});
    }

    return ret;
}

int MPI_Alltoall(const void *sendbuf, int sendcount, MPI_Datatype sendtype, void *recvbuf,
                 int recvcount, MPI_Datatype recvtype, MPI_Comm comm) {
    int ret;

    if (is_initialized == 1) {
        send_req_to_rapid(fd, htonl(INADDR_LOOPBACK), get_rapid_port(),
                (struct Request){.t = REQ_COMM_BEGIN, .pid = pid});
    }
    ret = PMPI_Alltoall(sendbuf, sendcount, sendtype, recvbuf, recvcount, recvtype, comm);
    if (is_initialized == 1) {
        send_req_to_rapid(fd, htonl(INADDR_LOOPBACK), get_rapid_port(),
                (struct Request){.t = REQ_COMM_END, .pid = pid});
    }

    return ret;
}

int MPI_Wait(MPI_Request *request, MPI_Status *status) {
    int ret;

    if (is_initialized == 1) {
        send_req_to_rapid(fd, htonl(INADDR_LOOPBACK), get_rapid_port(),
                (struct Request){.t = REQ_WAIT_BEGIN, .pid = pid});
    }
    ret = PMPI_Wait(request, status);
    if (is_initialized == 1) {
        send_req_to_rapid(fd, htonl(INADDR_LOOPBACK), get_rapid_port(),
                (struct Request){.t = REQ_WAIT_END, .pid = pid});
    }

    return ret;
}

int MPI_Waitall(int count, MPI_Request array_of_requests[],
                MPI_Status *array_of_statuses) {
    int ret;

    if (is_initialized == 1) {
        send_req_to_rapid(fd, htonl(INADDR_LOOPBACK), get_rapid_port(),
                (struct Request){.t = REQ_WAIT_BEGIN, .pid = pid});
    }
    ret = PMPI_Waitall(count, array_of_requests, array_of_statuses);
    if (is_initialized == 1) {
        send_req_to_rapid(fd, htonl(INADDR_LOOPBACK), get_rapid_port(),
                (struct Request){.t = REQ_WAIT_END, .pid = pid});
    }

    return ret;
}

int MPI_Allreduce(const void *sendbuf, void *recvbuf, int count, MPI_Datatype datatype,
                  MPI_Op op, MPI_Comm comm) {
    int ret;

    if (is_initialized == 1) {
        send_req_to_rapid(fd, htonl(INADDR_LOOPBACK), get_rapid_port(),
                (struct Request){.t = REQ_COMM_BEGIN, .pid = pid});
    }
    ret = PMPI_Allreduce(sendbuf, recvbuf, count, datatype, op, comm);
    if (is_initialized == 1) {
        send_req_to_rapid(fd, htonl(INADDR_LOOPBACK), get_rapid_port(),
                (struct Request){.t = REQ_COMM_END, .pid = pid});
    }

    return ret;
}

int create_udp_socket() {
    int fd;

    fd = socket(AF_INET, SOCK_DGRAM, 0);
    if (fd == -1)
        return -1;

    return fd;
}

in_port_t get_rapid_port() {
    uint16_t port_host_order;
    char *rapid_port_env = getenv("RAPID_PORT");
    if (rapid_port_env == NULL) {
        port_host_order = RAPID_DEFAULT_PORT;
    } else {
        port_host_order = atoi(rapid_port_env);
    }
    return htons(port_host_order);
}

int send_req_to_rapid(int fd, in_addr_t rapid_addr, in_port_t rapid_port,
                      struct Request req) {
    ssize_t n_sent;
    struct sockaddr_in saddr;

    saddr.sin_family = AF_INET;
    saddr.sin_addr.s_addr = rapid_addr;
    saddr.sin_port = rapid_port;
    n_sent = sendto(fd, (char *)&req, RAPID_REQUEST_SIZE, 0, (struct sockaddr *)&saddr,
                    sizeof(saddr));
    if (n_sent == -1)
        return -1;

    return n_sent;
}
