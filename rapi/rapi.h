#ifndef RAPILIB_H
#define RAPILIB_H

#include <netinet/in.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>

#define RAPID_REQUEST_SIZE 8
#define RAPID_DEFAULT_PORT 8210

enum RequestType {
    REQ_UNREGISTER = 0,
    REQ_REGISTER = 1,
    REQ_STOP = 2,
    REQ_CONT = 3,
    REQ_BEGIN_COMM = 4,
    REQ_END_COMM = 5,
};

struct Request {
    enum RequestType t;
    pid_t pid;
};

int send_req_to_rapid(int fd, in_addr_t rapid_addr,
                      in_port_t rapid_port, struct Request req);

int create_udp_socket();
in_port_t get_rapid_port();

#endif
