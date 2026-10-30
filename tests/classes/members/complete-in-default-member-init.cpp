// tidal-rules: CLS-2
// tidal-expect: run
struct S {
  int a = b + 1;
  static constexpr int b = 4;
};
int main() { return S{}.a == 5 ? 0 : 1; }
