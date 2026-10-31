// tidal-rules: MPTR-11
// tidal-expect: reject
// tidal-error-clang: type .int \*. with an rvalue of type .int S::\*.
// tidal-error-gcc: cannot convert .int S::\*. to .int\*.
// tidal-error-edg: int S::\*. cannot be used to initialize an entity of type .int \*
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
int* p = &S::a;
