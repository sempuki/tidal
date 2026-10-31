// tidal-rules: MPTR-10
// tidal-expect: reject
// tidal-error-clang: is not a function or function pointer
// tidal-error-gcc: must use .\.\*. or .->\*. to call pointer-to-member function
// tidal-error-edg: must have \(pointer-to-\) function type
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
int g(S s, int (S::* pf)(int) const) { return s.*pf(1); }
