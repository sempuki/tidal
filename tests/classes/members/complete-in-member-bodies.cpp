// tidal-rules: CLS-2
// tidal-expect: run
struct S {
  int get() const { return later; }
  int later = 7;
};
int main() { return S{}.get() == 7 ? 0 : 1; }
