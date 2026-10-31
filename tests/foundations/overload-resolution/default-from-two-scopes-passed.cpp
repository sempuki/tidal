// tidal-rules: OVL-38
// tidal-expect: accept
// tidal-note: The same declarations as default-from-two-scopes.cpp. The call passes the argument, so no default argument is used.
namespace low { extern "C" void tidal_ovl_f(int = 5); }
namespace high { extern "C" void tidal_ovl_f(int = 7); }
using low::tidal_ovl_f;
using high::tidal_ovl_f;
void use() { tidal_ovl_f(3); }
