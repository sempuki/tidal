// tidal-rules: MPTR-10
// tidal-expect: reject
// tidal-error-clang: reference to non-static member function must be called
// tidal-error-gcc: unable to deduce .auto. from .s\.\*pf.
// tidal-error-edg: pointer to a bound function may only be used to call
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
auto g(S s, int (S::* pf)(int) const) { auto x = s.*pf; return 0; }
