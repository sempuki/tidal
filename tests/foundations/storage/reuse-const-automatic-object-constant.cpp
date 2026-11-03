// tidal-rules: STG-20
// tidal-expect: reject
// tidal-expect-clang-18: accept
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: modification of object of const-qualified type
// tidal-note: Q-0307. Creating an object in the storage of a const automatic object is undefined, so constant evaluation must reject it. Clang 22 rejects. GCC 16 and EDG 7.0 accept and read the old value. Clang 18 and GCC 14 accept and read the new value.
#include "construct.h"
constexpr int reuse() {
  const int limit = 1;
  std::construct_at(const_cast<int*>(&limit), 2);
  return limit;
}
static_assert(reuse() == 1 || reuse() == 2);
