// tidal-rules: VOL-12
// tidal-expect: run
// tidal-flags: -O2
int seed() { return 9; }
volatile int fixed = 7;
volatile int zeroed;
volatile int computed = seed();
int local() {
  volatile int start = 5;
  volatile int from_call = seed();
  return start + from_call;
}
int main() { return fixed == 7 && zeroed == 0 && computed == 9 && local() == 14 ? 0 : 1; }
