// tidal-rules: ODR-13
// tidal-expect: run
// tidal-sources: explicit-specialization-declared-in-header.tu.cpp
#include "count-template.h"
int main() {
  return digits(5L) == 19 && digits_elsewhere() == 19 && digits(5) == 0 ? 0 : 1;
}
