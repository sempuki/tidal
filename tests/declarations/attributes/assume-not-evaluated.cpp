// tidal-rules: ATTR-15
// tidal-expect: run
int main() {
  int i = 1;
  [[assume(++i > 0)]];
  return i == 1 ? 0 : 1;
}
