// tidal-rules: THR-1
// tidal-expect: run
int copies = 0;
int moves = 0;
struct S { S() {} S(const S&) { ++copies; } S(S&&) { ++moves; } };

void raise(S s) { throw s; }  // no try block in raise

int main() {
  try { raise(S{}); } catch (const S&) { return copies == 0 && moves == 1 ? 0 : 1; }
  return 2;
}
