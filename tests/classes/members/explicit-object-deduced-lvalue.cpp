// tidal-rules: CLS-13
// tidal-expect: run
// tidal-note: Roadmap round 7, programmer reduction xobj_template_ref.cpp and xobj_auto_ref.cpp (reports/clean-room/roadmap-7/programmer.md, section 1.5). Self& and auto& take the object of the call, also in a call with a qualified name.
struct G {
  int v = 0;
  template <class Self> int get(this Self& self) { return self.v; }
  int put(this auto& self, int n) { self.v = n; return n; }
};
int main() {
  G g;
  g.put(7);
  const G& c = g;
  return g.get() == 7 && c.get() == 7 && g.G::get() == 7 ? 0 : 1;
}
