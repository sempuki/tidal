// tidal-rules: NAM-37 PFX-11
// tidal-expect: reject
// tidal-error-gcc: has no member named .scale.
// tidal-error-clang: no member named .scale. in
// tidal-error-edg: has no member .scale.
// tidal-note: Member lookup does not find a free function, also one in the namespace of the class.
namespace units {
  struct Meter { int v; };
  int scale(Meter m) { return m.v; }
}
int main() { units::Meter m{0}; return m.scale(); }
