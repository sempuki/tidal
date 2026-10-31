// tidal-rules: THR-8
// tidal-expect: run
int copies = 0;
int moves = 0;
int dtors = 0;
struct S { int v; S(int x) : v(x) {} S(const S& o) : v(o.v) { ++copies; } S(S&& o) : v(o.v) { ++moves; } ~S() { ++dtors; } };

int main() {
  try { try { throw S(1); } catch (const S&) { throw; } } catch (const S&) { if (dtors != 0) return 1; }
  return dtors == 1 ? 0 : 1; }
