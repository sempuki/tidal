// tidal-rules: STG-9
// tidal-expect: accept
// tidal-note: The new int takes the place of the member m of the base subobject. STG-9 replaces m, because the old and the new int are direct subobjects of one Base object. Every compiler gives the new value through the old name. The gap review of the object model reads the C++23 text the other way (documentation/decisions/pending.md, row 4).
#include "construct.h"
struct Base { int m; };
struct Derived : Base { int n; };
constexpr int replaced() {
  Derived d{{1}, 2};
  std::construct_at(&d.m, 5);
  return d.m;
}
static_assert(replaced() == 5);
