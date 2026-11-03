// tidal-rules: STMT-7
// tidal-expect: run
// tidal-note: The statements name a volatile local variable and a volatile reference. The test shows the statements are valid; it cannot observe the reads.
int main() {
  volatile int local = 2;
  volatile int& ref = local;
  local;
  ref;
  return ref == 2 ? 0 : 1;
}
