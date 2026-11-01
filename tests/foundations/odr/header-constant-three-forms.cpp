// tidal-rules: LNK-3 INL-1 INC-1
// tidal-expect: run
// tidal-sources: header-constant-three-forms.tu.cpp
#include "shared-limit.h"
static_assert(one_limit == 8 && each_limit == 8);
int main() {
  if (one_limit_in_other_unit() != &one_limit) return 1;   // one object
  if (each_limit_in_other_unit() == &each_limit) return 2; // one per unit
  if (far_limit != 8) return 3;                            // read at run time
  return 0;
}
