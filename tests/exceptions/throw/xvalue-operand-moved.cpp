// tidal-rules: THR-1
// tidal-expect: run
// tidal-note: The operand is an xvalue, a cast of an lvalue to an rvalue reference. The variable is outside the try block, and the exception object is still moved.
int copies = 0;
int moves = 0;
struct S { int v; S(int x) : v(x) {} S(const S& o) : v(o.v) { ++copies; } S(S&& o) : v(o.v) { ++moves; } };
int main() { S outside(6); try { throw static_cast<S&&>(outside); } catch (const S& e) { return e.v == 6 && copies == 0 && moves == 1 ? 0 : 1; } return 2; }
