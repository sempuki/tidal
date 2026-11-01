// tidal-rules: PFX-11 NAM-6
// tidal-expect: reject
// tidal-error-gcc: has no member named .later.
// tidal-error-clang: member access into incomplete type
// tidal-error-edg: has no member .later.|incomplete type
// tidal-note: The member declaration of b sees only the members declared before it, so the access does not find later.
struct Gauge {
  decltype(static_cast<Gauge*>(nullptr)->later) b;
  int later;
};
