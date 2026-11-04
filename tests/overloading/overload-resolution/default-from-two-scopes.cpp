// tidal-rules: OVL-38
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-expect-edg: accept
// tidal-error-gcc: default argument mismatch in overload resolution
// tidal-note: Q-0140. The one function f is declared in two namespaces, each with a default argument, and the call uses the default.
namespace low { extern "C" void tidal_ovl_f(int = 5); }
namespace high { extern "C" void tidal_ovl_f(int = 7); }
using low::tidal_ovl_f;
using high::tidal_ovl_f;
void use() { tidal_ovl_f(); }
