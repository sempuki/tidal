// tidal-rules: THR-1
// tidal-expect: run
int copies = 0;
int moves = 0;
struct S { int v; S(int x) : v(x) {} S(const S& o) : v(o.v) { ++copies; } S(S&& o) : v(o.v) { ++moves; } };

int main() {
  S local(6);  // declared outside the try block, so it outlives the try block
  try { throw local; } catch (const S& e) { return e.v == 6 && copies == 1 && moves == 0 ? 0 : 1; }
  return 2;
}
