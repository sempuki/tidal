// tidal-rules: THR-1
// tidal-expect: run
int copies = 0;
int moves = 0;
struct S { S() {} S(const S&) { ++copies; } S(S&&) { ++moves; } };

int main() {
  try {
    S local;  // in the outer try block, outside the inner one
    try { throw local; } catch (int) {}
  } catch (const S&) { return copies == 1 && moves == 0 ? 0 : 1; }
  return 2;
}
