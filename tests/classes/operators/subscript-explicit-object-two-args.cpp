// tidal-rules: OPR-3 CLS-13 CLS-15
// tidal-expect: run
// tidal-note: Roadmap round 7, programmer reduction xobj_subscript_two_args.cpp (reports/clean-room/roadmap-7/programmer.md, section 1.5). One operator[] with a deduced object parameter serves a non-const and a const object.
struct G {
  int cells[4]{};
  template <class Self> constexpr auto&& operator[](this Self&& self, int r, int c) {
    return static_cast<Self&&>(self).cells[r * 2 + c];
  }
};
int main() {
  G g{};
  g[1, 1] = 7;
  const G& c = g;
  return g[1, 1] == 7 && c[1, 1] == 7 && c[0, 0] == 0 ? 0 : 1;
}
