// End one element with an explicit destructor call and build it again,
// in constant evaluation, through an array member of a union.
#include "construct.h"
struct Num { int v; constexpr Num(int x) : v(x) {} constexpr ~Num() {} };
union Row { Num items[2]; constexpr Row() {} constexpr ~Row() {} };
constexpr int f() {
  Row r;
  std::construct_at(&r.items[0], 1);
  std::construct_at(&r.items[1], 2);
  r.items[0].~Num();
#if CASE == 1
  return r.items[0].v;          // read after the element ended
#else
  std::construct_at(&r.items[0], 5);
  int s = r.items[0].v + r.items[1].v;
  r.items[0].~Num(); r.items[1].~Num();
  return s;
#endif
}
static_assert(f() > 0);
int main() {}
