// tidal-rules: ODR-12
// tidal-expect: run
// tidal-sources: header-function-template-shared.tu.cpp
#include "twice-template.h"
int main() {
  tally<int>() = 4;
  bool one_function = &twice<int> == twice_elsewhere();
  bool one_local = &tally<int>() == tally_elsewhere() && *tally_elsewhere() == 4;
  return twice(6) == 12 && one_function && one_local ? 0 : 1;
}
