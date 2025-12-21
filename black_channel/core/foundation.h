#ifndef _BC_FOUNDATION_H_
#define _BC_FOUNDATION_H_
#include <stdint.h>
#include <stdio.h>
#include <signal.h>
#include <stdlib.h>
#include <errno.h>

#define __BC_CHECK_TYPE_ERROR -0xFFF0
#define __BC_CHECK_TYPE_WARN -0xFFF1
#define __BC_CHECK_TYPE_DEPRECATED -0xFFF2
#define __BC_FAILURE -1

#define BC_DEBUG

#define BC_ERROR(__message, ...)                                               \
	printf("[BC][Error]: ");                                               \
	printf(__message, ##__VA_ARGS__);                                      \
	printf("\n");                                                          \
	printf("ERRNO = %s\n", strerror(errno));                               \
	printf("FILE = %s\n", __FILE__);                                       \
	printf("LINE = %d\n", __LINE__);                                       \
	raise(SIGTRAP);

#define BC_CHECK(__condition, __type, __message, ...)                          \
	if (__condition) {                                                     \
		if (__type == __BC_CHECK_TYPE_ERROR) {                         \
			printf("[BC][Error]: ");                               \
			printf(__message, ##__VA_ARGS__);                      \
			printf("\n");                                          \
			printf("ERRNO = %s\n", strerror(errno));               \
			printf("FILE = %s\n", __FILE__);                       \
			printf("LINE = %d\n", __LINE__);                       \
			raise(SIGTRAP);                                        \
		} else if (__type == __BC_CHECK_TYPE_WARN) {                   \
			printf("[BC][Warning]: ");                             \
			printf(__message, ##__VA_ARGS__);                      \
			printf("\n");                                          \
			printf("FILE = %s\n", __FILE__);                       \
			printf("LINE = %d\n", __LINE__);                       \
		} else {                                                       \
			printf("[BC]: ");                                      \
			printf(__message, ##__VA_ARGS__);                      \
			printf("\n");                                          \
			printf("FILE = %s\n", __FILE__);                       \
			printf("LINE = %d\n", __LINE__);                       \
		}                                                              \
	}

#define BC_LOG(_message, ...)                                                  \
	printf("[BC]: ");                                                      \
	printf(_message, ##__VA_ARGS__);                                       \
	printf("\n");

#endif