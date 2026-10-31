// tidal-rules: THR-1
// tidal-expect: run
int copies = 0;
int moves = 0;
int dtors = 0;
struct S { int v; S(int x) : v(x) {} S(const S& o) : v(o.v) { ++copies; } S(S&& o) : v(o.v) { ++moves; } ~S() { ++dtors; } };

int main() { try { S local(6); throw local; } catch (const S& e) { return e.v == 6 && moves == 1 && copies == 0 ? 0 : 1; } return 2; }
