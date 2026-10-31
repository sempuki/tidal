// tidal-rules: OVL-27 OVL-32
// tidal-expect: accept
// tidal-note: OVL-32 prefers the written operator!= template. OVL-27 prefers the rewritten !(a == b) from a plain function. The rewritten candidate wins, so OVL-27 comes first.
struct Item {};
template <class T> constexpr bool operator!=(const T&, const Item&) { return false; }
constexpr bool operator==(const Item&, const Item&) { return false; }
static_assert(Item{} != Item{});
