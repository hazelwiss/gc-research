#include "../single-task.h"
uint8_t task_data[] = {
	#embed "sr-out-of-bounds.bin"
};
uint8_t result_data_[] = {
	#embed "../results/sr-out-of-bounds.result.bin"
};
uint8_t *result_data = result_data_;
#include "../test-main.h"
