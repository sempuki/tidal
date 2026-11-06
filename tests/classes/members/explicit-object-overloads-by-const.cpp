// tidal-rules: CLS-9 CLS-13
// tidal-expect: run
// tidal-note: Write suite W18, finding W18-2 (reports/clean-room/W18/report.md). Two member functions whose explicit object parameters are V& and const V& are two functions, and a call picks by the constness of the object.
struct V {
  int f(this V&) { return 1; }
  int f(this const V&) { return 2; }
};
int main() {
  V v;
  const V& c = v;
  return v.f() == 1 && c.f() == 2 ? 0 : 1;
}
