// tidal-rules: LNK-9
// tidal-expect: run
int value = 1;
int f() {
  int value = 2;
  return value;
}
int g() {
  static int value = 3;
  return value;
}
int main() { return value == 1 && f() == 2 && g() == 3 ? 0 : 1; }
