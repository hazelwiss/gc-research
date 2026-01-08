#include "../single-task.h"
uint8_t task_data[] = {
	#embed "srrd-out-of-bounds.bin"
};
uint8_t result_data_[] = {
	#embed "../results/srrd-out-of-bounds.result.bin"
};
uint8_t *result_data = result_data_;
#include "../test-main.h"
