// tidal-rules: MPTR-10
// tidal-expect: reject
// tidal-error-clang: can only be called on an rvalue
// tidal-error-gcc: requires an rvalue
// tidal-error-edg: can only be used with an rvalue object
struct R { int f() && { return 1; } };
int h(R& r, int (R::* pf)() &&) { return (r.*pf)(); }
