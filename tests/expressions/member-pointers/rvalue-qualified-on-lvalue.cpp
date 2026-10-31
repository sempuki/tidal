// tidal-rules: MPTR-3
// tidal-expect: reject
struct R { int f() && { return 1; } };
int h(R& r, int (R::* pf)() &&) { return (r.*pf)(); }
