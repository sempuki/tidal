// tidal-rules: CLS-13 DED-8
// tidal-expect: run
// tidal-note: Roadmap round 7, programmer reduction xobj_template_fwd_value.cpp (reports/clean-room/roadmap-7/programmer.md, section 1.5). Self deduces as for a forwarding reference: G& from a non-const lvalue, const G& from a const lvalue, G from an rvalue.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct G {
  int v = 0;
  template <class Self> int kind(this Self&&) {
    if constexpr (same<Self, G&>) return 1;
    else if constexpr (same<Self, const G&>) return 2;
    else if constexpr (same<Self, G>) return 3;
    else return 0;
  }
};
int main() {
  G g;
  const G& c = g;
  return g.kind() == 1 && c.kind() == 2 && G{}.kind() == 3 ? 0 : 1;
}
