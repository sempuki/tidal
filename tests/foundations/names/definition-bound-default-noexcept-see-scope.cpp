// tidal-rules: NAM-21
// tidal-expect: accept
// tidal-note: After the qualified name, an array bound, a default argument, and a noexcept specifier see the members of the namespace or class.
namespace N { constexpr int k = 2; extern int a[k]; }
int N::a[k];
struct C { static const int n = 3; static int b[n]; void f(int = n) noexcept(n > 0); };
int C::b[n];
void C::f(int) noexcept(n > 0) {}
