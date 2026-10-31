// tidal-rules: MPTR-10
// tidal-expect: reject
// tidal-error-clang: must be a class compatible with the right hand operand
// tidal-error-gcc: incompatible with object type .U.
// tidal-error-edg: selection class types are incompatible
struct S { int a = 1; int b = 2; int f(int x) const { return a + x; } static int s; };
int S::s = 0;
struct U { int a; };
int g(U u, int S::* pm) { return u.*pm; }
