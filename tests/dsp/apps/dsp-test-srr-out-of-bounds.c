#include "../single-task.h"
uint8_t task_data[] = {
	#embed "srr-out-of-bounds.bin"
};
uint8_t result_data_[] = {
	#embed "../results/srr-out-of-bounds.result.bin"
};
uint8_t *result_data = result_data_;
#include "../test-main.h"
