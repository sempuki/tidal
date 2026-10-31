// tidal-rules: NEW-5
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-note: Gap review 2026-10-31. Q-0091. GCC accepts delete[] of a derived array through a base pointer during constant evaluation. Clang and EDG reject it.
struct B { int b = 0; constexpr virtual ~B() {} };
struct D : B { int d = 0; };
constexpr bool f() { B* p = new D[2]; delete[] p; return true; }
static_assert(f());
