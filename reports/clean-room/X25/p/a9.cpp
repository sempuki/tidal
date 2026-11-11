// A test of validity sees whether the construction of an element of an
// array member that is not active is a constant. main returns 1 when
// the constant is valid and 0 when it is not.
#include "construct.h"
template <class T>
union Row { T items[2]; constexpr Row() {} };
template <class T>
constexpr T build() {
  Row<T> r;
  return *std::construct_at(&r.items[1], T(7));
}
template <int V> struct Value {};
template <class T>
constexpr bool is_constant = requires { typename Value<build<T>()>; };
int main() { return is_constant<int> ? 1 : 0; }
