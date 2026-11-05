// tidal-rules: STO-9
// tidal-expect: run-fail
// tidal-error: recursive_init_error
// tidal-sources: recursive-static-init-across-units.tu.cpp
// tidal-note: The initializer of the block-scope variable in left calls right in the other unit, and right's initializer calls left again. Control reaches left's declaration while its initialization runs. The run-time library of Clang and GCC detects the second entry and ends the program.
int& right();
int& left() {
  static int value = right() + 1;
  return value;
}
int main() { return left(); }
