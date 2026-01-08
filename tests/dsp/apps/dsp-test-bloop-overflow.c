#include "../single-task.h"
uint8_t task_data[] = {
	#embed "bloop-overflow.bin"
};
uint8_t result_data_[] = {
	#embed "../results/bloop-overflow.result.bin"
};
uint8_t *result_data = result_data_;
#include "../test-main.h"
