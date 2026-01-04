#include "../single-task.h"
uint8_t task_data[] = {
	#embed "lsrnrx.bin"
};
uint8_t result_data_[] = {
	#embed "../results/lsrnrx.result.bin"
};
uint8_t *result_data = result_data_;
#include "../test-main.h"
