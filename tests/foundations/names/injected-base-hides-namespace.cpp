// tidal-rules: NAM-35
// tidal-expect: accept
struct base { int v = 1; };
namespace lib { struct base { int w = 2; }; struct derived : ::base { int get() { base b; return b.v; } }; }
