#include "../single-task.h"
uint8_t task_data[] = {
	#embed "srrn-lrrn.bin"
};
uint8_t *result_data = 0;
#include "../test-main.h"
