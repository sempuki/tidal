// A constexpr variable whose class holds started and unstarted elements
// at the end of the constant evaluation.
#include "construct.h"
struct Vec {
  union { int items[4]; };
  int count = 0;
  constexpr Vec() {}
  constexpr void push(int x) { std::construct_at(items + count, x); ++count; }
};
constexpr Vec make() { Vec v; v.push(1); v.push(2); return v; }
#if CASE == 1
constexpr Vec kept = make();
static_assert(kept.items[1] == 2);
#else
constexpr int f() { Vec v = make(); return v.items[0] + v.items[1]; }
static_assert(f() == 3);
#endif
int main() {}
