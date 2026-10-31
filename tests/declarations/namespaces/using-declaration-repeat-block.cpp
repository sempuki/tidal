// tidal-rules: NS-7
// tidal-expect: accept
// tidal-expect-gcc-14: reject
// tidal-expect-edg: reject
// tidal-error-gcc: redeclaration of .int N::x
// tidal-error-edg: x. has already been declared in the current scope
// tidal-note: Q-0019. Clang 22 and 18 and GCC 16 accept a repeated using declaration in a block; GCC 14 and EDG 7.0 reject it.
namespace N { int x; }
void f() {
  using N::x;
  using N::x;
}
