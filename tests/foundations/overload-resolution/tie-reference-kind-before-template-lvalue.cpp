// tidal-rules: OVL-29 OVL-27
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: Q-0148. The mirror of tie-reference-kind-before-template.cpp. OVL-29 prefers the template, which returns an lvalue reference. OVL-27 prefers the plain operator F&&. GCC chooses the template, and EDG the plain function. Clang also chooses the template, because it gives an lvalue reference no conversion that returns an rvalue reference (Q-0138).
using F = int();
constexpr int a() { return 1; }
constexpr int b() { return 2; }
struct Source {
  constexpr operator F&&() const { return a; }
  template <class T> constexpr operator T&() const { return b; }
};
constexpr F& r = Source{};
static_assert(r() == 2);
