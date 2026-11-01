#include "shared-limit.h"
const int far_limit = 8;
static_assert(far_limit == 8);
const int* one_limit_in_other_unit() { return &one_limit; }
const int* each_limit_in_other_unit() { return &each_limit; }
