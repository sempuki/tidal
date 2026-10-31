// tidal-rules: NAM-27
// tidal-expect: accept
namespace c { struct root { }; int probe(const root&) { return 1; } }
namespace b { struct base : c::root { }; }
namespace d { struct derived : b::base { }; }
int read() { return probe(d::derived{}); }
