// tidal-rules: MPTR-10
// tidal-expect: reject
struct R { int f() & { return 1; } };
int g(int (R::* pf)() &) { return (R{}.*pf)(); }
