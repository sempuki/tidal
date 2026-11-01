// tidal-rules: ODR-12 ODR-13
// tidal-expect: run
// tidal-sources: header-specializations-shared.tu.cpp
#include "kind-template.h"
int main() {
  Kind<int*>::count = 5;
  Kind<char>::count = 6;
  bool kinds = Kind<int>::value() == 0 && Kind<int*>::value() == 1 && Kind<char>::value() == 2;
  bool partial = &Kind<int*>::count == pointer_count_elsewhere() && *pointer_count_elsewhere() == 5;
  bool explicit_class = &Kind<char>::count == char_count_elsewhere() && *char_count_elsewhere() == 6;
  bool explicit_function = &width<char> == width_elsewhere() && width('a') == 1 && width(0) == 4;
  bool explicit_variable = &base_of<char> == char_base_elsewhere() && base_of<char> == 16;
  return kinds && partial && explicit_class && explicit_function && explicit_variable ? 0 : 1;
}
