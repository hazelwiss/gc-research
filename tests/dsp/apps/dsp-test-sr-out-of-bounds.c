#include "../single-task.h"
uint8_t task_data[] = {
	#embed "sr-out-of-bounds.bin"
};
uint8_t *result_data = 0;
#include "../test-main.h"
