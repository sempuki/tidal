// tidal-rules: THR-3 THR-1
// tidal-expect: run
// tidal-note: The class has a move constructor and a deleted copy constructor. The throw moves the local variable, and only the move constructor needs to be usable.
struct S { int v; S(int x) : v(x) {} S(const S&) = delete; S(S&& o) : v(o.v) {} };
int main() { try { S local(6); throw local; } catch (const S& e) { return e.v == 6 ? 0 : 1; } return 2; }
