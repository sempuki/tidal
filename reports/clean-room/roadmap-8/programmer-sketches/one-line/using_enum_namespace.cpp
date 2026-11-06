enum class Color { red, green };
using enum Color;
int main() { Color c = green; return c == Color::green ? 0 : 1; }
