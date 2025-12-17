#pragma once
#include <stddef.h>
#include <stdint.h>

struct __attribute__((packed)) state {
  uint16_t gpr[32];
};

typedef void(*callback_t)(struct state*) ;
typedef void(*callback_new_test_t)(const char*, uint32_t) ;
typedef void(*callback_skipped_test_t)(void) ;

void run(callback_t, callback_new_test_t, callback_skipped_test_t, size_t print_start);
