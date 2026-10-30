// tidal-rules: DCL-3
// tidal-expect: run
int main() {
  int x = 1;
  int& r = x;
  r = 5;
  return x == 5 && &r == &x ? 0 : 1;
}
