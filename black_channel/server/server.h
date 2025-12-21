#ifndef _BC_SERVER_H_
#define _BC_SERVER_H_

#include <stdint.h>
#include <openssl/ssl.h>

/// -------------------------------------------------------------------------
/// server utils

/*
Description
-----------
Creates Linux/POSIX compatible sockets. 

Returns
-------
int32_t
	File Descriptor for the created Socket. FD is always valid,
	Raises SIGTRAP if something goes wrong.

Note
----
Sets the following socket options,
	SO_REUSEADDR, SO_REUSEPORT, TCP_NODELAY, IPV6_V6ONLY
*/
int32_t bc_create_server_socket();

/*
Description
-----------
Binds valid Socket FD to the port & Listens to them. Socket must be
created using `__bc_create_server_socket`.

Args
----
const int32_t *const __p_server_fd
	VALID server socket file descriptor

const int16_t __port
	VALID 16-bit port

const int32_t __backlog
	server backlog
*/
void bc_bind_and_listen_server_socket(const int32_t *const __p_server_fd,
				      const int16_t __port,
				      const int32_t __backlog);

/*
Description
-----------
Creates an epoll instance.

Returns
-------
int32_t
	File descriptor for the epoll instance. FD is always valid. 
	Raises SIGTRAP if something goes wrong.
*/
int32_t bc_create_event_poll_instance();

/*
Description
-----------
Adds the socket file descriptor the the epoll instances with event options

Args
----
const int32_t *const __p_epoll_fd
	VALID epoll instances' file descriptor

const int32_t *const __p_socket_fd
	VALID socket file descriptor

int32_t __event_options
	Event options for the struct `epoll_event`
*/
void bc_add_socket_to_epoll_instance(const int32_t *const __p_epoll_fd,
				     const int32_t *const __p_socket_fd,
				     int32_t __event_options);

/*
Description
-----------
Deletes the socket file descriptor the the epoll instances with event options

Args
----
const int32_t *const __p_epoll_fd
	VALID epoll instances' file descriptor

const int32_t *const __p_socket_fd
	VALID socket file descriptor
*/
void bc_delete_socket_to_epoll_instance(const int32_t *const __p_epoll_fd,
					const int32_t *const __p_socket_fd);

/// -------------------------------------------------------------------------
/// client utils

/*
Description
-----------
Reads the data from client FD

Args
----
const int32_t *const __p_epoll_fd
	VALID epoll instances' file descriptor

const int32_t *const __p_server_fd
	VALID server socket file descriptor

const int32_t *const __p_client_fd
	VALID client socket file descriptor

const int64_t __buffer_size
	Buffer size, Reads in `__buffer_size` periods.
*/
void bc_read_data(const int32_t *const __p_epoll_fd,
		  const int32_t *const __p_server_fd,
		  const int32_t *const __p_client_fd,
		  const int64_t __buffer_size);

/*
Description
-----------
Accepts the client socket from epoll instance.

Args
----
const int32_t *const __p_epoll_fd
	VALID epoll instances' file descriptor

const int32_t *const __p_server_fd
	VALID server socket file descriptor

const int32_t *const __p_client_fd
	VALID client socket file descriptor

int32_t __client_epoll_event_options
	Event options for the struct `epoll_event`
*/
void bc_accept_client_socket(const int32_t *const __p_epoll_fd,
			     const int32_t *const __p_server_fd,
			     int32_t __client_epoll_event_options);

/// -------------------------------------------------------------------------
/// generic utils

/*
Description
-----------
Closes the File Descriptor

Args
----
int32_t __fd
	VALID & open file descriptor to close.
*/
void bc_close_fd(int32_t __fd);

/// structures

typedef struct bc_server_config_t {
	int32_t max_events; // epoll event queue
	int32_t timeout; // server socket timeout
	int32_t port; // server socket port
	int32_t backlog; // server backlog period
	int64_t buffer_size; // read buffer size
} bc_server_config_t;

typedef struct bc_connection_t {
	int32_t max_events; // epoll event queue
	int32_t timeout; // server socket timeout
	int32_t port; // server socket port
	int32_t backlog; // server backlog period
	int64_t buffer_size; // read buffer size
} bc_connection_t;

/// server functions

/*
Description
-----------
Creates and Initialises the server

Args
----
bc_server_config_t *__server_config
	server configuration
*/
void bc_server_init_and_poll(bc_server_config_t *__server_config);

// secure versions
// void __bc_accept_socket_linux_secure(int32_t *_p_socket_fd,
// 				     int32_t *_p_epoll_fd,
// 				     SSL_CTX *_global_ssl_contex,
// 				     SSL **_global_client_fd_map);
// void __bc_read_data_secure(int32_t *_p_client_fd, int32_t *_p_server_fd,
// 			   const int64_t _buffer_size,
// 			   SSL **_global_ssl_client_fd_map);

#endif