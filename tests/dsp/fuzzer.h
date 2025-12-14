#pragma once
#include <stdint.h>

struct state {
  uint16_t gpr[32];
};

typedef void(*callback_t)(struct state*) ;

void run(callback_t);
