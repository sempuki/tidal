// tidal-rules: STG-19 LSUP-10
// tidal-expect: reject
// tidal-error-gcc: accessing .U::whole. member instead of (initialized|active) .U::real. member
// tidal-error-clang: read of member .* of union with active member
// tidal-error-edg: expression must have a constant value
#include "construct.h"
union U { int whole; float real; };
constexpr int read_old() {
  U u{.whole = 1};
  int& old = u.whole;
  std::construct_at(&u.real, 2.0f);
  return old;
}
static_assert(read_old() == 1);
