// tidal-rules: UNI-6
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-gcc: member instead of (active|initialized)
// tidal-note: Q-0088. EDG accepts assignment through a user-provided operator to an inactive union member during constant evaluation. Clang and GCC reject it.
struct T { int v; constexpr T& operator=(int x) { v = x; return *this; } };
union U { int i; T t; };
constexpr int f() { U u{.i = 1}; u.t = 5; return u.t.v; }
static_assert(f() == 5);
