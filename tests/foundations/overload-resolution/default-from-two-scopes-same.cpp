// tidal-rules: OVL-38
// tidal-expect: accept
// tidal-note: Q-0140. The two namespaces give the same default argument, so the call is valid.
namespace low { extern "C" void tidal_ovl_g(int = 5); }
namespace high { extern "C" void tidal_ovl_g(int = 5); }
using low::tidal_ovl_g;
using high::tidal_ovl_g;
void use() { tidal_ovl_g(); }
