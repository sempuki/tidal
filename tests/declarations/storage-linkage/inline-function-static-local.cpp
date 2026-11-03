// tidal-rules: LNK-4
// tidal-expect: run
// tidal-sources: inline-function-static-local.tu.cpp
// tidal-note: Both units define the same inline function. Its block-scope static variable is one object, so a change in one unit shows in the other.
inline int& counter() {
  static int value = 0;
  return value;
}
void bump_other();
int main() {
  counter() = 4;
  bump_other();
  return counter() == 5 ? 0 : 1;
}
