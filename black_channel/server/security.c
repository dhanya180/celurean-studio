#include <black_channel/core/foundation.h>
#include <black_channel/server/security.h>
#include <openssl/err.h>

SSL_CTX *__bc_create_ssl_context(const char *_cert_file_path,
				 const char *_key_file_path)
{
	// initialise OpenSSL
	SSL_load_error_strings();
	OpenSSL_add_ssl_algorithms();

	// create new context
	SSL_CTX *ssl_context;
	ssl_context = SSL_CTX_new(TLS_server_method());

	// sanity check
	if (ssl_context == NULL) {
		ERR_print_errors_fp(stderr);
		BC_ERROR("failed to create openssl context");
	}

	// load certificate file
	int32_t result = SSL_CTX_use_certificate_file(
		ssl_context, _cert_file_path, SSL_FILETYPE_PEM);
	BC_CHECK(result == __BC_FAILURE, __BC_CHECK_TYPE_ERROR,
		 "failed to load/use ssl certificate from file");

	// load key file
	result = SSL_CTX_use_PrivateKey_file(ssl_context, _key_file_path,
					     SSL_FILETYPE_PEM);
	BC_CHECK(result == __BC_FAILURE, __BC_CHECK_TYPE_ERROR,
		 "failed to load/use ssl certificate from file");

	// return
	BC_LOG("ssl initialised, certificate & key loaded");
	return ssl_context;
}
