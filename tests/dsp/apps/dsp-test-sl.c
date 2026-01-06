#include "../single-task.h"
uint8_t task_data[] = {
	#embed "sl.bin"
};
uint8_t *result_data = 0;
#include "../test-main.h"
