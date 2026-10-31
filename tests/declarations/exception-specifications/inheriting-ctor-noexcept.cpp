// tidal-rules: EXS-4
// tidal-expect: accept
struct B { B(int) noexcept {} B(double) {} };
struct D : B { using B::B; };
static_assert(noexcept(D(1)));
static_assert(!noexcept(D(1.0)));
