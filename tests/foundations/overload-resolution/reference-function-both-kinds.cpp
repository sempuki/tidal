// tidal-rules: OVL-15
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: non-const lvalue reference to type .F. .* cannot bind to a temporary of type .Source.
// tidal-note: Q-0138. A reference to function takes conversion functions of both kinds, so operator F&&() serves an lvalue reference to F. Clang does not take it.
using F = int();
constexpr int a() { return 1; }
struct Source { constexpr operator F&&() const { return a; } };
constexpr F& r = Source{};
static_assert(r() == 1);
