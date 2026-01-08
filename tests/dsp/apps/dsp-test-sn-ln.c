#include "../single-task.h"
uint8_t task_data[] = {
	#embed "sn-ln.bin"
};
uint8_t result_data_[] = {
	#embed "../results/sn-ln.result.bin"
};
uint8_t *result_data = result_data_;
#include "../test-main.h"
