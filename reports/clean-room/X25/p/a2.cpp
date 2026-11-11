// After std::construct_at builds element 0 of an array member that was
// not active, the program reads element 1, which no construction built.
#include "construct.h"
union Row { int items[2]; constexpr Row() {} };
constexpr int f() {
  Row r;
  std::construct_at(&r.items[0], 1);
#if CASE == 1
  return r.items[1];            // read of an element that was never built
#elif CASE == 2
  r.items[1] = 5;               // assignment to an element that was never built
  return r.items[1];
#elif CASE == 3
  return r.items[0];            // read of the built element through the member
#elif CASE == 4
  Row s = r;                    // trivial copy of the union
  return s.items[0];
#endif
}
static_assert(f() >= 0);
int main() {}
