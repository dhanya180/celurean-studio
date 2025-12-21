#include <black_channel/server/server.h>

int main(int argc, char *argv[])
{
	struct bc_server_config_t server_config;
	memset(&server_config, 0, sizeof(server_config));
	server_config.backlog = 1024;
	server_config.buffer_size = 1024;
	server_config.max_events = 1024;
	server_config.port = 8000;
	server_config.timeout = 300;

	bc_server_init_and_poll(&server_config);
}
