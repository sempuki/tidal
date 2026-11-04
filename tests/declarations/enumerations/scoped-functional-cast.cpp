// tidal-rules: ENUM-3
// tidal-expect: accept
// tidal-note: Propose exercise X20. F(5) is a cast with one operand, which tries static_cast (CAST-9). So the integer converts to the scoped enumeration.
enum class F : int { a, b, c, d, e, g };
constexpr F f = F(5);
static_assert(f == F::g);
static_assert(static_cast<int>(F(2)) == 2);
