// tidal-rules: NAM-27
// tidal-expect: accept
namespace b { struct base { }; int probe(const base&) { return 1; } }
namespace d { struct derived : b::base { }; }
int read() { return probe(d::derived{}); }
