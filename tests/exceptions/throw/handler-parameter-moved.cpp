// tidal-rules: THR-1
// tidal-expect: run
// tidal-note: The operand names the parameter of the handler that holds the throw. No try block in the handler encloses the throw, so the new exception object is moved from the parameter.
int copies = 0;
int moves = 0;
struct S { int v; S(int x) : v(x) {} S(const S& o) : v(o.v) { ++copies; } S(S&& o) : v(o.v) { ++moves; } };
int main() {
  try {
    try { throw S(6); }
    catch (S e) { copies = 0; moves = 0; throw e; }
  } catch (const S& e) { return e.v == 6 && copies == 0 && moves == 1 ? 0 : 1; }
  return 2;
}
