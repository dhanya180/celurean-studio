#ifndef _BC_SERVER_SECURITY_H_
#define _BC_SERVER_SECURITY_H_

#include <openssl/ssl.h>

/**
 * @brief Creates SSL context, used by `bc_server_init`. Parameters are not
 * checked for nullptrs. Caller Headache.
 * 
 * @param _cert_file_path certificate (*.pem) file path
 * @param _key_file_path private key (*.pem) file path
 * @return SSL_CTX* 
 */
SSL_CTX *__bc_create_ssl_context(const char *_cert_file_path,
				 const char *_key_file_path);

#endif
