// tidal-rules: OVL-38
// tidal-expect: accept
// tidal-note: Q-0140. The default arguments 5, (5), and 0x5 are one expression: parentheses do not count, and the literals have the same value. So the call that uses the default is valid.
namespace low { extern "C" void tidal_ovl_m(int = 5); }
namespace mid { extern "C" void tidal_ovl_m(int = (5)); }
namespace high { extern "C" void tidal_ovl_m(int = 0x5); }
using low::tidal_ovl_m;
using mid::tidal_ovl_m;
using high::tidal_ovl_m;
void use() { tidal_ovl_m(); }
