// tidal-rules: CLS-13
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
struct S {
  int v = 4;
  int get(this S self) { return self.v; }
  int twice(this const S& self) { return self.v * 2; }
};
int main() {
  S s;
  return S{}.get() == 4 && s.twice() == 8 ? 0 : 1;
}
