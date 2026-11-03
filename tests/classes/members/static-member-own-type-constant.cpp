// tidal-rules: CLS-16
// tidal-expect: accept
// tidal-note: Write reports W2 and W7. The member is const in the class and inline constexpr after it.
class Color {
  int code;
public:
  constexpr explicit Color(int c) : code(c) {}
  constexpr int value() const { return code; }
  static const Color red, green;
};
inline constexpr Color Color::red{1};
inline constexpr Color Color::green{2};
static_assert(Color::red.value() == 1);
constexpr Color pick(bool warm) { return warm ? Color::red : Color::green; }
static_assert(pick(false).value() == 2);
