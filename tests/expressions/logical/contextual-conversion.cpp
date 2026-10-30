// tidal-rules: LOG-2
// tidal-expect: accept
struct Flag { bool v; constexpr explicit operator bool() const { return v; } };
constexpr int x = 0;
static_assert(Flag{true} && 2);
static_assert(!Flag{false});
static_assert(0.5 || false);
static_assert(!0.0);
static_assert(&x && !static_cast<const int*>(nullptr));
