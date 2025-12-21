project('black_channel', 'c')

source_dir = 'black_channel'
include_dirs = include_directories([
    'black_channel'
])

# required by sys/socket.h to expose accept4
add_project_arguments('-D_GNU_SOURCE', language: 'c')

executable(
    'black_channel', 
    [
        'black_channel/server/server.c',
        #'black_channel/server/security.c',
        'black_channel/main.c'
    ],
    include_directories: include_dirs,
)
