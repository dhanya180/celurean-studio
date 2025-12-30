#include <black_channel/core/foundation.h>
#include <black_channel/server/server.h>
#include <stdint.h>
#include <errno.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/epoll.h>
#include <netinet/in.h>
#include <netinet/tcp.h>

int32_t bc_create_server_socket()
{
	// server socket creation
	int32_t server_fd = socket(AF_INET6, SOCK_STREAM | SOCK_NONBLOCK, 0);
	// sanity check
	BC_CHECK(server_fd == __BC_FAILURE, __BC_CHECK_TYPE_ERROR,
		 "server creation failed");
	BC_LOG("created ipv6 streaming & non-blocking socket [%d]", server_fd);

	// setsockopt configuration
	int32_t socket_option_on = 1, socket_option_off = 0;
	// set SO_REUSEADDR
	int32_t result = setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR,
				    &socket_option_on, sizeof(int32_t));
	BC_CHECK(result == __BC_FAILURE, __BC_CHECK_TYPE_ERROR,
		 "setsockopt - SO_REUSEADDR failed");

	// set SO_REUSEPORT (mulithreading)
	result = setsockopt(server_fd, SOL_SOCKET, SO_REUSEPORT,
			    &socket_option_on, sizeof(int32_t));
	BC_CHECK(result == __BC_FAILURE, __BC_CHECK_TYPE_ERROR,
		 "setsocketopt - SO_REUSEPORT failed");

	// set IPV6ONLY
	result = setsockopt(server_fd, IPPROTO_IPV6, IPV6_V6ONLY,
			    &socket_option_off, sizeof(int32_t));
	BC_CHECK(result == __BC_FAILURE, __BC_CHECK_TYPE_ERROR,
		 "setsocketopt - IPV6_V6ONLY failed");

	// set TCP_NODELAY
	result = setsockopt(server_fd, IPPROTO_TCP, TCP_NODELAY,
			    &socket_option_on, sizeof(int32_t));
	BC_CHECK(result == __BC_FAILURE, __BC_CHECK_TYPE_ERROR,
		 "setsocketopt - TCP_NODELAY failed");

	BC_LOG("server created with fd = %d", server_fd);
	return server_fd;
}

void bc_bind_and_listen_server_socket(const int32_t *const __p_server_fd,
				      const int16_t __port,
				      const int32_t __backlog)
{
	// parameter extraction
	const int32_t server_fd = *__p_server_fd;

	// binding socket structures
	struct sockaddr_in6 socket_in_address;
	// crucial; garbage data in socket address can cause undefined errors
	memset(&socket_in_address, 0, sizeof(struct sockaddr_in6));
	socket_in_address.sin6_family = AF_INET6;
	socket_in_address.sin6_port = htons(__port);
	socket_in_address.sin6_addr = in6addr_any;

	// bind server to port
	int32_t result = bind(server_fd, (struct sockaddr *)&socket_in_address,
			      sizeof(socket_in_address));

	BC_CHECK(result == __BC_FAILURE, __BC_CHECK_TYPE_ERROR,
		 "binding server [%d] to port [%d] failed", server_fd, __port);
	BC_LOG("bound server [%d] to port %d", server_fd, __port);

	// set server to listening mode
	result = listen(server_fd, __backlog);
	BC_CHECK(result == __BC_FAILURE, __BC_CHECK_TYPE_ERROR,
		 "server[%d] failed on being set to listen with backlog [%d]",
		 server_fd, __backlog);
	BC_LOG("server [%d] is actively listening | backlog = %d", server_fd,
	       __backlog);
}

void bc_close_fd(int32_t __fd)
{
	BC_CHECK(close(__fd) == __BC_FAILURE, __BC_CHECK_TYPE_ERROR,
		 " unable to close the file descriptor [%d]", __fd);
	BC_LOG("closing file descriptor [%d]", __fd);
}

int32_t bc_create_event_poll_instance()
{
	// create epoll instance
	int32_t epoll_fd = epoll_create1(EPOLL_CLOEXEC);
	BC_CHECK(epoll_fd == __BC_FAILURE, __BC_CHECK_TYPE_ERROR,
		 "epoll creation failed");
	BC_LOG("epoll instance created with fd = %d", epoll_fd);

	return epoll_fd;
}

void bc_add_socket_to_epoll_instance(const int32_t *const __p_epoll_fd,
				     const int32_t *const __p_socket_fd,
				     int32_t __event_options)
{
	// argument extraction
	const int32_t socket_fd = *__p_socket_fd;
	const int32_t epoll_fd = *__p_epoll_fd;

	// add the server to epoll controller
	struct epoll_event socket_event;
	// crucial, any garbage memory here would mess up socket event and
	// give unexpected results
	memset(&socket_event, 0, sizeof(socket_event));

	socket_event.events = __event_options;
	socket_event.data.fd = socket_fd;

	// add server to epoll instance
	int32_t result =
		epoll_ctl(epoll_fd, EPOLL_CTL_ADD, socket_fd, &socket_event);
	BC_CHECK(result == __BC_FAILURE, __BC_CHECK_TYPE_ERROR,
		 "adding socket [%d] to epoll instance [%d] failed", socket_fd,
		 epoll_fd);
	BC_LOG("added socket [%d] to epoll instance [%d]", socket_fd, epoll_fd);
}

void bc_delete_socket_to_epoll_instance(const int32_t *const __p_epoll_fd,
					const int32_t *const __p_socket_fd)
{
	// argument extraction
	const int32_t socket_fd = *__p_socket_fd;
	const int32_t epoll_fd = *__p_epoll_fd;

	// delete socket fd from epoll instance
	int32_t result = epoll_ctl(epoll_fd, EPOLL_CTL_DEL, socket_fd, NULL);
	BC_CHECK(result == __BC_FAILURE, __BC_CHECK_TYPE_ERROR,
		 "deleting socket [%d] from epoll instance [%d] failed",
		 socket_fd, epoll_fd);
	// close fd
	bc_close_fd(socket_fd);
	BC_LOG("deleted socket [%d] from epoll instance [%d]", socket_fd,
	       epoll_fd);
}

void bc_accept_client_socket(const int32_t *const __p_epoll_fd,
			     const int32_t *const __p_server_fd,
			     int32_t __client_epoll_event_options)
{
	// argument extraction
	const int32_t server_fd = *__p_server_fd;
	const int32_t epoll_fd = *__p_epoll_fd;

	// ipv6 client address structs
	struct sockaddr_in6 client_address;
	int32_t client_address_len = sizeof(client_address);

	for (;;) {
		// non blocking client connection
		int32_t client_fd =
			accept4(server_fd, (struct sockaddr *)&client_address,
				(socklen_t *)&client_address_len,
				SOCK_NONBLOCK | SOCK_CLOEXEC);

		// may indicate no clients (or) error, check & branch
		if (client_fd == __BC_FAILURE) {
			// no clients, then break from loop
			// if (errno == EAGAIN || errno == EWOULDBLOCK) {
			// 	BC_LOG("no clients to connect to", errno);
			// 	break;
			// }
			if (errno == EAGAIN || errno == EWOULDBLOCK) {
				BC_LOG("no clients to connect to: %s\n", strerror(errno));
				break;
			}
			// error
			else {
				BC_ERROR(
					"accepting client from server [%d] over epoll instance [%d] failed",
					server_fd, epoll_fd);
			}
		}

		BC_LOG("accepted new client [%d] to server [%d] over epoll instance [%d]",
		       client_fd, server_fd, epoll_fd);

		// set TCP_NODELAY (disables nagales algorithm)
		int32_t socket_option_on = 1;
		int32_t result = setsockopt(client_fd, IPPROTO_TCP, TCP_NODELAY,
					    &socket_option_on, sizeof(int32_t));
		BC_CHECK(result == __BC_FAILURE, __BC_CHECK_TYPE_ERROR,
			 "setsockopt - TCP_NODELAY failed to client [%d]",
			 client_fd);

		// add to epoll instance
		bc_add_socket_to_epoll_instance(__p_epoll_fd, &client_fd,
						__client_epoll_event_options);
	}
}

void bc_read_data(const int32_t *const __p_epoll_fd,
		  const int32_t *const __p_server_fd,
		  const int32_t *const __p_client_fd,
		  const int64_t __buffer_size)
{
	// argument extraction
	const int32_t server_fd = *__p_server_fd;
	const int32_t epoll_fd = *__p_epoll_fd;
	const int32_t client_fd = *__p_client_fd;

	//data buffer
	char buffer[__buffer_size];
	for (;;) {
		// clear buffer
		memset(buffer, 0, __buffer_size * sizeof(char));
		ssize_t bytes_read = recv(client_fd, buffer, __buffer_size, 0);

		// something was read
		if (bytes_read > 0) {
			BC_LOG("received %s from client [%d]", buffer,
			       client_fd);
		}
		// client connection closed (breaks)
		else if (bytes_read == 0) {
			bc_delete_socket_to_epoll_instance(&epoll_fd,
							   &client_fd);
			break;
		}
		// maybe nothing to read, check & branch
		else {
			// nothing to read
			if (errno == EAGAIN || errno == EWOULDBLOCK) {
				break;
			}
			// error
			else {
				BC_ERROR(
					"error reading bytes from client[%d] over epoll instance [%d] | server [%d]",
					client_fd, epoll_fd, server_fd);
			}
		}
	}
}

void bc_server_init_and_poll(bc_server_config_t *__server_config)
{
	// initialise server
	int32_t server_fd = bc_create_server_socket();
	bc_bind_and_listen_server_socket(&server_fd,
					 (uint16_t)__server_config->port,
					 __server_config->backlog);

	// initialise epoll instance
	int32_t epoll_fd = bc_create_event_poll_instance();

	// epoll event queue
	struct epoll_event events[__server_config->max_events];
	memset(events, 0, sizeof(events));

	int32_t client_event_options = EPOLLIN | EPOLLRDHUP | EPOLLET;

	for (;;) {
		int32_t polled_fd_count =
			epoll_wait(epoll_fd, events,
				   __server_config->max_events,
				   __server_config->timeout);
		// maybe interrupt, check for it and branch to error
		if (polled_fd_count == __BC_FAILURE) {
			if (errno == EINTR) {
				// interrupt signal
				continue;
			} else {
				BC_ERROR("epoll_wait failed");
			}
		}

		BC_LOG("received [%d] connection requets from server [%d] over epoll instance [%d]",
		       polled_fd_count, server_fd, epoll_fd);

		for (int32_t i = 0; i < polled_fd_count; ++i) {
			int32_t received_fd = events[i].data.fd;

			// received connection request
			if (received_fd == server_fd) {
				BC_LOG("received new connection request to [%d]",
				       server_fd);
				bc_accept_client_socket(&server_fd, &epoll_fd,
							client_event_options);
			}
			// we have received data from existing client connection
			else {
				int32_t client_fd = events[i].data.fd;
				// events error
				if (events[i].events &
				    ((EPOLLERR | EPOLLHUP | EPOLLRDHUP))) {
					BC_LOG("epoll event error");
					bc_delete_socket_to_epoll_instance(
						&epoll_fd, &client_fd);
					break;
				}
				BC_LOG("data received from client [%d] to [%d]",
				       events[i].data.fd, server_fd);
				bc_read_data(&epoll_fd, &server_fd, &client_fd,
					     __server_config->buffer_size);
			}
		}
	}

	// cleanup
	bc_close_fd(server_fd);
	bc_close_fd(epoll_fd);
}

// void __bc_accept_client_linux_secure(int32_t *_p_socket_fd,
// 				     int32_t *_p_epoll_fd,
// 				     SSL_CTX *_global_ssl_contex,
// 				     SSL **_global_ssl_client_fd_map)
// {
// 	struct sockaddr_in6 client_address;
// 	int32_t client_address_len = sizeof(client_address);
// 	for (;;) {
// 		// non blocking client connection
// 		int32_t client_fd = accept4(*_p_socket_fd,
// 					    (struct sockaddr *)&client_address,
// 					    (socklen_t *)&client_address_len,
// 					    SOCK_NONBLOCK | SOCK_CLOEXEC);

// 		if (client_fd == -1) { // maybe error, check and branch
// 			if (errno == EAGAIN || errno == EWOULDBLOCK) {
// 				BC_LOG("accept4 returned errno = [%d]", errno);
// 				break;
// 			} else { // definitely error!
// 				BC_CHECK(
// 					true, _bc_CHECK_TYPE_ERROR,
// 					"accepting client from server [%d] over epoll instance [%d] failed",
// 					*_p_socket_fd, *_p_epoll_fd);
// 			}
// 		}

// 		BC_LOG("accepted new client [%d] to server [%d] over epoll instance [%d]",
// 		       client_fd, *_p_socket_fd, *_p_epoll_fd);

// 		// create & map ssl object
// 		SSL *client_ssl_object = SSL_new(_global_ssl_contex);
// 		SSL_set_fd(client_ssl_object, client_fd);
// 		SSL_set_accept_state(client_ssl_object);

// 		int return_code = SSL_accept(client_ssl_object);
// 		if (return_code <= 0) {
// 			int error =
// 				SSL_get_error(client_ssl_object, return_code);
// 			if (error != SSL_ERROR_WANT_READ &&
// 			    error != SSL_ERROR_WANT_WRITE) {
// 				BC_LOG("ssl handshake failed for client [%d]",
// 				       client_fd);

// 				SSL_free(client_ssl_object);
// 				_bc_close_fd_linux(client_fd);
// 				continue;
// 			}
// 		}
// 		// map to global client file descriptor ssl map
// 		_global_ssl_client_fd_map[client_fd] = client_ssl_object;

// 		// add client to epoll control
// 		struct epoll_event socket_event;
// 		memset(&socket_event, 0, sizeof(struct epoll_event));

// 		socket_event.events = EPOLLIN | EPOLLET | EPOLLRDHUP;
// 		socket_event.data.fd = client_fd;

// 		BC_CHECK(epoll_ctl(*_p_epoll_fd, EPOLL_CTL_ADD, client_fd,
// 				   &socket_event) == _bc_FAILURE,
// 			 _bc_CHECK_TYPE_ERROR,
// 			 "adding (_%d) to epoll instalce (%d) failed",
// 			 client_fd, *_p_epoll_fd);
// 		BC_LOG("added socket (%d) to epoll instance (%d)", client_fd,
// 		       *_p_epoll_fd);
// 	}
// }

// void __bc_read_data_secure(int32_t *_p_client_fd, int32_t *_p_server_fd,
// 			  const int64_t _buffer_size,
// 			  SSL **_global_ssl_client_fd_map)
// {
// 	char buffer[_buffer_size];
// 	SSL *ssl_object = _global_ssl_client_fd_map[*_p_client_fd];

// 	for (;;) {
// 		// init buffer to zero
// 		memset(buffer, 0, _buffer_size * sizeof(char));
// 		ssize_t bytes_read =
// 			SSL_read(ssl_object, buffer, sizeof(buffer) - 1);

// 		if (bytes_read > 0) {
// 			BC_LOG("received %s from client [%d]", buffer,
// 			       *_p_client_fd);
// 		} else {
// 			int error = SSL_get_error(ssl_object, bytes_read);
// 			if (error == SSL_ERROR_WANT_READ ||
// 			    error == SSL_ERROR_WANT_WRITE) {
// 				// buffer was empty
// 				break;
// 			} else {
// 				// error
// 				BC_LOG("client [%d] disconnected, fatal error",
// 				       *_p_client_fd);

// 				SSL_shutdown(ssl_object);
// 				SSL_free(ssl_object);
// 				_global_ssl_client_fd_map[*_p_client_fd] = NULL;
// 				_bc_close_fd_linux(*_p_client_fd);
// 			}
// 		}
// 	}
// }