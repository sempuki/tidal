// tidal-rules: THR-1
// tidal-expect: run
// tidal-note: The operand names a local rvalue reference, so the exception object is moved from the object that the reference names.
int copies = 0;
int moves = 0;
struct S { int v; S(int x) : v(x) {} S(const S& o) : v(o.v) { ++copies; } S(S&& o) : v(o.v) { ++moves; } };
S make() { return S(6); }
int main() { try { S&& r = make(); throw r; } catch (const S& e) { return e.v == 6 && copies == 0 && moves == 1 ? 0 : 1; } return 2; }
