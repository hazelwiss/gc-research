#include "../single-task.h"
uint8_t task_data[] = {
	#embed "orc.bin"
};
uint8_t result_data_[] = {
	#embed "../results/orc.result.bin"
};
uint8_t *result_data = result_data_;
#include "../test-main.h"
