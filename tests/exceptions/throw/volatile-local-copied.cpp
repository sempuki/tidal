// tidal-rules: THR-1
// tidal-expect: run
// tidal-note: The operand names a volatile local variable, so the exception object is copied. The move constructor cannot take a volatile object.
int copies = 0;
int moves = 0;
struct S { int v; S(int x) : v(x) {} S(const volatile S& o) : v(o.v) { ++copies; } S(S&& o) : v(o.v) { ++moves; } };
int main() { try { volatile S local(6); throw local; } catch (const S& e) { return e.v == 6 && copies == 1 && moves == 0 ? 0 : 1; } return 2; }
