// tidal-rules: UNI-5
// tidal-expect: accept
struct S { int v; constexpr S(int x) : v(x) {} constexpr ~S() {} };
union U { int i; S s; constexpr U() : i(0) {} constexpr ~U() {} };
constexpr int f() { U u; u.i = 2; return u.i; }
static_assert(f() == 2);
