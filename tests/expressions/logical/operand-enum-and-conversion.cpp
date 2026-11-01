// tidal-rules: LOG-2
// tidal-expect: accept
enum Color { red, green };
struct Count { constexpr operator int() const { return 2; } };
static_assert(!red && green);
static_assert(Count{} && true);
static_assert(!(red || Count{} == 0));
