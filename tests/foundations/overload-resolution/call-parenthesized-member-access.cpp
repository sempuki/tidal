// tidal-rules: OVL-21 OVL-4 PFX-20
// tidal-expect: run
// tidal-note: (x.f)(a) is the call x.f(a), with x as the implied object argument. The parentheses do not make the name an unqualified call, so the free function scale in the namespace of the argument is not a candidate.
namespace units {
  struct Meter { int v; };
  int scale(Meter) { return 1; }
}
struct Gauge {
  int scale(units::Meter m) const { return m.v + 2; }
  int twice(this const Gauge&, int a) { return a * 2; }
};
int main() {
  Gauge g;
  const Gauge* p = &g;
  units::Meter m{1};
  if ((g.scale)(m) != 3) return 1;
  if ((p->scale)(m) != 3) return 2;
  return (g.twice)(4) == 8 ? 0 : 3;
}
