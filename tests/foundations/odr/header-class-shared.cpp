// tidal-rules: ODR-7 INC-4
// tidal-expect: run
// tidal-sources: header-class-shared.tu.cpp
#include "square.h"
#include "square.h"
int main() {
  Square s{3};
  bool same_object = made_address() == made_address_elsewhere();
  return s.area() == 9 && perimeter(s) == 12 && same_object ? 0 : 1;
}
