// tidal-rules: VOL-12
// tidal-expect: run
// tidal-flags: -O2
// tidal-note: A member initializer writes a volatile member, and a braced initializer writes the elements of a volatile array, at run time.
int seed() { return 6; }
struct Register {
  volatile int value;
  Register(int v) : value(v) {}
};
int main() {
  Register r(seed());
  volatile int pair[2] = {seed(), 2};
  return r.value == 6 && pair[0] == 6 && pair[1] == 2 ? 0 : 1;
}
