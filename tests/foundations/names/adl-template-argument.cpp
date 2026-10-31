// tidal-rules: NAM-27
// tidal-expect: accept
namespace t { template <class X> struct box { }; }
namespace n { struct S { }; int probe(t::box<S>) { return 1; } }
int read() { return probe(t::box<n::S>{}); }
