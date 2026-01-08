#include "../single-task.h"
uint8_t task_data[] = {
	#embed "sl.bin"
};
uint8_t result_data_[] = {
	#embed "../results/sl.result.bin"
};
uint8_t *result_data = result_data_;
#include "../test-main.h"
