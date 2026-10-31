// tidal-rules: MPTR-3
// tidal-expect: accept
struct R { int f() const& { return 1; } };
int g(int (R::* pf)() const&) { return (R{}.*pf)(); }
