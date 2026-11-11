// using enum in a switch over a scoped enumeration.
enum class Color { red, green, blue };
constexpr int code(Color c) {
  switch (c) {
    using enum Color;
    case red: return 1;
    case green: return 2;
    case blue: return 3;
  }
  return 0;
}
static_assert(code(Color::blue) == 3);
int main() { return code(Color::red) - 1; }
