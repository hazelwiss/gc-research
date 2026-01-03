char* files[] = {
    "dvd:/nop.bin"
};
char* result_files[] = {
	"dvd:/../results/nop.result.bin.bin"
};
int file_cnt = 1;

#define HAS_DISK
            #include "../all-tasks.h"
#include "../test-main.h"
