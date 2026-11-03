// tidal-rules: STO-9
// tidal-expect: run-fail
// tidal-error: recursive_init_error
// tidal-note: Gap review 2026-10-30. The run-time library of Clang and GCC detects the second entry and ends the program.
int depth = 0;
int make();
int get() {
  static int value = make();
  return value;
}
int make() { return ++depth < 3 ? get() : 1; }
int main() { return get(); }
