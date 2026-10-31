// tidal-rules: THR-8
// tidal-expect: run
int copies = 0;
int moves = 0;
int dtors = 0;
struct S { int v; S(int x) : v(x) {} S(const S& o) : v(o.v) { ++copies; } S(S&& o) : v(o.v) { ++moves; } ~S() { ++dtors; } };

int main() { try { throw S(2); } catch (S e) { if (copies + moves != 1) return 1; } return dtors == 2 ? 0 : 1; }
