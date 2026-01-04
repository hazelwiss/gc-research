#include "../single-task.h"
uint8_t task_data[] = {
	#embed "nr.bin"
};
uint8_t result_data_[] = {
	#embed "../results/nr.result.bin"
};
uint8_t *result_data = result_data_;
#include "../test-main.h"
