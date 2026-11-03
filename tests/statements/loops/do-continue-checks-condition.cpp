// tidal-rules: LOOP-2
// tidal-expect: run
// tidal-note: The body runs before the first test of the condition. A continue goes to the condition.
int checks = 0;
bool again(int n) { ++checks; return n < 3; }
int main() {
  int n = 0;
  int runs = 0;
  do {
    ++runs;
    ++n;
    if (n == 2) continue;
  } while (again(n));
  return runs == 3 && checks == 3 ? 0 : 1;
}
