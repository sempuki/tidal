// tidal-rules: DTY-5
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-note: Q-0065. GCC accepts decltype in the nested name of a member definition. Clang and EDG reject it.
struct S { void f(); };
S s;
void decltype(s)::f() {}
