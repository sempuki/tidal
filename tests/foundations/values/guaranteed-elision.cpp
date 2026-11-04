// tidal-rules: VC-4
// tidal-expect: run
struct NoCopy {
  int v;
  NoCopy(int x) : v(x) {}
  NoCopy(const NoCopy&) = delete;
  NoCopy(NoCopy&&) = delete;
};
NoCopy make() { return NoCopy(7); }
int main() {
  NoCopy a = NoCopy(NoCopy(5));
  NoCopy b = make();
  return a.v == 5 && b.v == 7 ? 0 : 1;
}
