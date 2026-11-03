// tidal-rules: CE-2 DER-12
// tidal-expect: reject
// tidal-error-clang: non-constexpr constructor .Left.|non-literal type .Left.
// tidal-error-gcc: call to non-.constexpr. function .Left::Left\(\)
// tidal-error-edg: cannot call non-constexpr function "Left::Left\(\)
// tidal-note: Write report W9. The implicit constructor of a class with a virtual base is not constexpr.
struct Base { int n = 0; };
struct Left : virtual Base {};
constexpr int make() {
  Left x;
  return x.n;
}
static_assert(make() == 0);
