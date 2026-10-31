// tidal-rules: NAM-27
// tidal-expect: accept
namespace t { template <class X> struct box { }; template <class X> int probe(box<X>) { return 1; } }
int read() { return probe(t::box<int>{}); }
