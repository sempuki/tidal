// tidal-rules: MPTR-10
// tidal-expect: reject
// tidal-error-clang: can only be called on an lvalue
// tidal-error-gcc: requires an lvalue
// tidal-error-edg: can only be used with an lvalue object
struct R { int f() & { return 1; } };
int g(int (R::* pf)() &) { return (R{}.*pf)(); }
