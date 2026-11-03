// tidal-rules: THR-1
// tidal-expect: run
int copies = 0;
int moves = 0;
struct S { S() {} S(const S&) { ++copies; } S(S&&) { ++moves; } };

bool caught = false;
void raise(S s) {
  try { throw s; } catch (const S&) { caught = true; }  // s outlives the try block
}

int main() {
  raise(S{});
  return caught && copies == 1 && moves == 0 ? 0 : 1;
}
