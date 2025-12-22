#include "fuzzer.h"
#include <stdio.h>

static void cbk(size_t id, struct state *result) {}

static void cbk_new_test(size_t id, const char *name, uint32_t cnt) {}

static void cbk_print_init(uint32_t total_tasks) {
  printf("Total tasks: %u\n", total_tasks);
}

static void cbk_print_update(struct task *task, uint64_t difftime,
                             uint32_t total_tasks, uint32_t task_id,
                             uint32_t total_cases, uint32_t case_id) {
  printf("\x1b[%d;0H", 2);
  printf("seconds lapsed %lu\n", difftime);
  printf("complete: %d / %u\n", task_id, total_tasks);
  printf("processing %s %d / %u...\n", task->name, case_id, total_cases);
}

int main() {
  run(cbk, cbk_new_test, cbk_print_init, cbk_print_update);
}
