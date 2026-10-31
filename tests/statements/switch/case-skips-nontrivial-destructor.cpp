// tidal-rules: SW-5
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-note: Q-0097. GCC accepts a jump past a variable whose destructor is not trivial. Clang and EDG reject it.
// tidal-note: Gap review 2026-10-31 (third).
struct D { int a; ~D() {} };
int f(int v) { switch (v) { case 0: D d; d.a = 1; return d.a; case 1: return 7; } return 0; }
