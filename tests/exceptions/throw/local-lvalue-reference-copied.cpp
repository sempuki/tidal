// tidal-rules: THR-1
// tidal-expect: run
// tidal-note: The operand names a local reference that is not an rvalue reference, so the exception object is copied.
int copies = 0;
int moves = 0;
struct S { int v; S(int x) : v(x) {} S(const S& o) : v(o.v) { ++copies; } S(S&& o) : v(o.v) { ++moves; } };
int main() { try { S local(6); S& r = local; throw r; } catch (const S& e) { return e.v == 6 && copies == 1 && moves == 0 ? 0 : 1; } return 2; }
