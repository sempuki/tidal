// tidal-rules: MPTR-9
// tidal-expect: reject
// tidal-error-clang: type .int T::\*. with an rvalue of type .int \*.
// tidal-error-gcc: cannot convert .int\*. to .int T::\*.
// tidal-error-edg: return value type does not match
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
struct T { int v; int T::* get() { return &v; } };
