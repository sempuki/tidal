// tidal-rules: COMMA-2
// tidal-expect: run
int x = 5;
int main() {
  int* p = &(1, x);
  int& r = (0.5, x);
  int&& m = (0, static_cast<int&&>(x));
  r = 6;
  return p == &x && &m == &x && x == 6 ? 0 : 1;
}
