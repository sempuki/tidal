// tidal-rules: CEV-2
// tidal-expect: accept
consteval int one() { return 1; }
consteval int call() { auto p = one; return p(); }
static_assert(call() == 1);
