// tidal-rules: NAM-28 NAM-8
// tidal-expect: reject
// tidal-error-clang: undeclared identifier .probe.
// tidal-error-gcc: probe. was not declared in this scope
// tidal-error-edg: identifier .probe. is undefined
namespace n { struct S { int v; }; }
namespace t { template <int X> struct box { }; }
namespace n { int probe(t::box<1>); }
int read() { return probe(t::box<1>{}); }
