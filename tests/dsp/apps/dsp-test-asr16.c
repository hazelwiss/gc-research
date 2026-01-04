#include "../single-task.h"
uint8_t task_data[] = {
	#embed "asr16.bin"
};
uint8_t result_data_[] = {
	#embed "../results/asr16.result.bin"
};
uint8_t *result_data = result_data_;
#include "../test-main.h"
