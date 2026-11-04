// tidal-rules: OVL-38
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-expect-edg: accept
// tidal-error-gcc: default argument mismatch in overload resolution
// tidal-note: Q-0140. The two default arguments 5 and 2 + 3 have the same value but are different expressions, so the call that uses the default is not valid. GCC rejects. Clang and EDG accept every call that uses default arguments from two scopes.
namespace low { extern "C" void tidal_ovl_k(int = 5); }
namespace high { extern "C" void tidal_ovl_k(int = 2 + 3); }
using low::tidal_ovl_k;
using high::tidal_ovl_k;
void use() { tidal_ovl_k(); }
