// tidal-rules: THR-3
// tidal-expect: accept
struct R { ~R() noexcept(false) {} };
void f() { throw R(); }
