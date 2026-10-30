// tidal-rules: COND-2
// tidal-expect: run
int a = 0, b = 0;
int left() { return ++a; }
int right() { return ++b; }
int main() {
  int r = true ? left() : right();
  int s = false ? left() : right();
  return r == 1 && s == 1 && a == 1 && b == 1 ? 0 : 1;
}
