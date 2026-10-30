// tidal-rules: REF-3
// tidal-expect: run
int make() { return 7; }
int main() {
  int x = 1;
  int&& a = 5;
  int&& b = make();
  int&& c = static_cast<int&&>(x);
  c = 9;
  a = 6;
  return a == 6 && b == 7 && x == 9 ? 0 : 1;
}
