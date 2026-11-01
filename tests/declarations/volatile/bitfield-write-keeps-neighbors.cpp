// tidal-rules: VOL-13
// tidal-expect: run
struct Control { unsigned enable : 1; unsigned mode : 3; unsigned rest : 28; };
int main() {
  volatile Control reg{};
  reg.enable = 1;
  reg.rest = 99;
  reg.mode = 5;                        // reads and writes the unit that holds mode
  return reg.enable == 1 && reg.mode == 5 && reg.rest == 99 ? 0 : 1;
}
