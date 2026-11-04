// tidal-rules: OVL-29 OVL-28
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-expect-edg: reject
// tidal-error-clang: static assertion failed
// tidal-error-edg: static assertion failed
// tidal-note: Q-0148. Two conversion function templates bind the rvalue function reference directly with the identity. OVL-29 prefers operator T&&. OVL-28 prefers operator T&, the more specialized template, which Clang chooses without OVL-29 (Q-0138). GCC chooses operator T&&, and EDG operator T&.
using F = int();
constexpr int a() { return 1; }
constexpr int b() { return 2; }
struct Source {
  template <class T> constexpr operator T&() const { return a; }
  template <class T> constexpr operator T&&() const { return b; }
};
constexpr F&& r = Source{};
static_assert(r() == 2);
