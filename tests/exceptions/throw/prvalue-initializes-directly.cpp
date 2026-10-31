// tidal-rules: THR-1
// tidal-expect: run
int copies = 0;
int moves = 0;
int dtors = 0;
struct S { int v; S(int x) : v(x) {} S(const S& o) : v(o.v) { ++copies; } S(S&& o) : v(o.v) { ++moves; } ~S() { ++dtors; } };

int main() { try { throw S(4); } catch (const S& e) { return e.v == 4 && copies == 0 && moves == 0 ? 0 : 1; } return 2; }
