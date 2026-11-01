// tidal-rules: ODR-12
// tidal-expect: run
// tidal-sources: header-class-template-shared.tu.cpp
#include "box-template.h"
int main() {
  Box<int> box{5};
  Box<int>::made = 3;
  bool one_member = &Box<int>::made == made_elsewhere() && *made_elsewhere() == 3;
  return box.get() == 5 && box.doubled() == 10 && doubled_elsewhere(box) == 10 && one_member ? 0 : 1;
}
