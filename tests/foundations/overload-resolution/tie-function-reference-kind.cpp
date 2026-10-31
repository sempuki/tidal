// tidal-rules: OVL-29
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: reference initialization of type .F &&. .* is ambiguous
// tidal-note: Q-0138. Both conversions bind the function reference directly with the identity, so no argument and no result sequence (OVL-26) decides, and neither is a template. The one that returns the same kind of reference as the target wins.
using F = int();
constexpr int a() { return 1; }
constexpr int b() { return 2; }
struct Source {
  constexpr operator F&() const { return a; }
  constexpr operator F&&() const { return b; }
};
constexpr F& left = Source{};
constexpr F&& right = Source{};
static_assert(left() == 1);
static_assert(right() == 2);
