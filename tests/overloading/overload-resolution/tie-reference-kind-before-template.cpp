// tidal-rules: OVL-29 OVL-27
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-expect-edg: reject
// tidal-error-clang: static assertion failed
// tidal-error-edg: static assertion failed
// tidal-note: Q-0148. Both results bind the rvalue function reference directly with the identity. OVL-29 prefers the template, which returns an rvalue reference. OVL-27 prefers the plain operator F&. GCC chooses the template. Clang and EDG choose the plain function.
using F = int();
constexpr int a() { return 1; }
constexpr int b() { return 2; }
struct Source {
  constexpr operator F&() const { return a; }
  template <class T> constexpr operator T&&() const { return b; }
};
constexpr F&& r = Source{};
static_assert(r() == 2);
