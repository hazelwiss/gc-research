#include "../single-task.h"
uint8_t task_data[] = {
	#embed "m2.bin"
};
uint8_t result_data_[] = {
	#embed "../results/m2.result.bin"
};
uint8_t *result_data = result_data_;
#include "../test-main.h"
