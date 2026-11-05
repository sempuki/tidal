// tidal-rules: OVL-38
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-expect-edg: accept
// tidal-error-gcc: default argument mismatch in overload resolution
// tidal-note: Q-0140. Each namespace declares its own variable n, and each default argument n names the variable of its namespace. The two arguments have one spelling and name different variables, so they are different expressions, and the call that uses the default is not valid. GCC rejects. Clang and EDG accept every call that uses default arguments from two scopes.
namespace low { int n = 1; extern "C" void tidal_ovl_p(int = n); }
namespace high { int n = 2; extern "C" void tidal_ovl_p(int = n); }
using low::tidal_ovl_p;
using high::tidal_ovl_p;
void use() { tidal_ovl_p(); }
