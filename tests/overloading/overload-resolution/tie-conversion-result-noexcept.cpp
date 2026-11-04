// tidal-rules: OVL-26
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: more than one
// tidal-note: Q-0163. The plain form of tie-conversion-result-templates.cpp. The result of operator Plain() is the identity, and the result of operator Safe() needs a function pointer conversion. EDG finds the two ambiguous.
using Plain = int (*)();
using Safe = int (*)() noexcept;
constexpr int a() { return 1; }
constexpr int b() noexcept { return 2; }
struct Source {
  constexpr operator Plain() const { return a; }
  constexpr operator Safe() const { return b; }
};
constexpr int (*p)() = Source{};
static_assert(p() == 1);
