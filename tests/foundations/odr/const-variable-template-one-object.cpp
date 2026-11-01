// tidal-rules: ODR-12
// tidal-expect: run
// tidal-expect-gcc-14: run-fail
// tidal-error-gcc-14: exit 3
// tidal-note: GCC 14 gives each unit its own lower<int> and upper<int> (Q-0297).
// tidal-sources: const-variable-template-one-object.tu.cpp
#include "limit-template.h"
int main() {
  bool one_lower = &lower<int> == lower_elsewhere();
  bool one_upper = &upper<int> == upper_elsewhere();
  return (one_lower ? 0 : 1) + (one_upper ? 0 : 2);
}
