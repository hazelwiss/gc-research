#pragma once
#include "tasks.h"
#include <stddef.h>
#include <stdint.h>

struct __attribute__((packed)) state {
  uint16_t gpr[32];
};

typedef void(*callback_case_done_t)(size_t, struct state*);
typedef void(*callback_new_test_t)(size_t, const char*, uint32_t);
typedef void(*callback_print_init_t)(uint32_t total_tasks);
typedef void(*callback_print_update_t)(struct task*, uint64_t difftime, uint32_t total_tasks, uint32_t task_id, uint32_t total_cases, uint32_t case_id);

void run(callback_case_done_t, callback_new_test_t, callback_print_init_t, callback_print_update_t);
