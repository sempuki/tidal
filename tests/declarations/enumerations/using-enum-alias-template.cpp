// tidal-rules: ENUM-9
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-note: RV10-3. A using enum declaration names an enumeration through an alias template, at namespace scope and in a block. Clang 18 rejects the template-id (Q-0020).
enum class Color { red, green };
template <class T> using Paint = Color;
using enum Paint<int>;
constexpr Color pick() { using enum Paint<char>; return red; }
static_assert(green == Color::green);
static_assert(pick() == Color::red);
