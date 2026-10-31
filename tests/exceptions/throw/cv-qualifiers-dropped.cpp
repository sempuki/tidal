// tidal-rules: THR-2
// tidal-expect: run
int copies = 0;
int moves = 0;
int dtors = 0;
struct S { int v; S(int x) : v(x) {} S(const S& o) : v(o.v) { ++copies; } S(S&& o) : v(o.v) { ++moves; } ~S() { ++dtors; } };

int main() { const S cs(3); try { throw cs; } catch (S& e) { e.v = 4; return e.v == 4 ? 0 : 1; } return 2; }
