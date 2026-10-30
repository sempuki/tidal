// tidal-rules: CPY-8
// tidal-expect: accept
struct S { int a; int b; };
constexpr S x{1, 2};
constexpr S y(x);
static_assert(y.b == 2);
