// tidal-rules: NAM-35
// tidal-expect: accept
namespace lib { struct base { int v = 1; }; }
struct derived : lib::base { int get() const { base copy = *this; return copy.v; } };
