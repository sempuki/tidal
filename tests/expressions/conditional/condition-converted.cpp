// tidal-rules: COND-1
// tidal-expect: accept
struct Flag { constexpr explicit operator bool() const { return true; } };
static_assert((Flag{} ? 1 : 2) == 1);
static_assert((0 ? 1 : 2) == 2);
constexpr int* np = nullptr;
static_assert((np ? 1 : 2) == 2);
