// using enum inside a switch over a scoped enumeration.
enum class Color { red, green, blue };
int code(Color c) {
  switch (c) {
    using enum Color;
    case red: return 1;
    case green: return 2;
    case blue: return 3;
  }
  return 0;
}
int main() { return code(Color::green) + code(Color::blue) == 5 ? 0 : 1; }
