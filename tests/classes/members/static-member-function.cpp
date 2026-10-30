// tidal-rules: CLS-7
// tidal-expect: run
struct S {
  static int make() { return 3; }
};
int main() {
  S s;
  return S::make() == 3 && s.make() == 3 ? 0 : 1;
}
