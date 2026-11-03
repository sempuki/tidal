// tidal-rules: STG-19
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-note: Q-0307. A new object in a base subobject does not replace the base (STG-9), so the read through the old pointer is undefined and constant evaluation must reject it. Clang 22, GCC 16, and EDG 7.0 accept the read and give the new value.
#include "construct.h"
struct Base { int value; constexpr Base(int v) : value(v) {} };
struct Derived : Base { constexpr Derived() : Base(1) {} };
constexpr int read_old() {
  Derived whole;
  Base* old = &whole;
  std::construct_at(old, 2);
  return old->value;
}
static_assert(read_old() == 2);
