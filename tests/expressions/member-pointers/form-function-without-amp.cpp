// tidal-rules: MPTR-9
// tidal-expect: reject
// tidal-error-clang: call to non-static member function without an object argument
// tidal-error-gcc: invalid use of non-static member function
// tidal-error-edg: nonstandard form for taking the address of a member function
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
int (S::* pf)(int) const = S::f;
