// tidal-rules: CLS-13 DED-8
// tidal-expect: accept
// tidal-note: Roadmap round 7, programmer reduction xobj_template_ref.cpp, in constant evaluation (reports/clean-room/roadmap-7/programmer.md, section 1.5).
struct G {
  int v = 0;
  template <class Self> constexpr int get(this const Self& self) { return self.v; }
  template <class Self> constexpr auto&& at(this Self&& self) { return static_cast<Self&&>(self).v; }
};
constexpr int f() {
  G g;
  g.at() = 5;
  return g.get() + G{}.at();
}
static_assert(f() == 5);
