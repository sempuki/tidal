// tidal-rules: OVL-38
// tidal-expect: accept
// tidal-note: Q-0140. The default arguments shared and ::shared have different spellings and name one variable, so they are the same expression, and the call that uses the default is valid.
int shared = 1;
namespace low { extern "C" void tidal_ovl_q(int = shared); }
namespace high { extern "C" void tidal_ovl_q(int = ::shared); }
using low::tidal_ovl_q;
using high::tidal_ovl_q;
void use() { tidal_ovl_q(); }
