// tidal-rules: ODR-12
// tidal-expect: run
// tidal-sources: header-variable-template-shared.tu.cpp
#include "start-template.h"
int main() {
  start_of<int> = 8;
  bool one_variable = &start_of<int> == start_elsewhere() && *start_elsewhere() == 8;
  return start_of<long> == 7 && one_variable ? 0 : 1;
}
