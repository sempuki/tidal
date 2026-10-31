// tidal-rules: OVL-27 OVL-32
// tidal-expect: accept
// tidal-note: The written candidate is a template, and the swapped candidate is a plain function. The swapped candidate wins, so OVL-27 comes first.
struct Left {};
struct Right {};
template <class T> constexpr bool operator==(const T&, const Right&) { return true; }
constexpr bool operator==(const Right&, const Left&) { return false; }
static_assert(!(Left{} == Right{}));
