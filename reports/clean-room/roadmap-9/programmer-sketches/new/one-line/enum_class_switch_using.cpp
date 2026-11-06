enum class Color : unsigned char { red, green, blue };
constexpr int code(Color c) { switch (c) { using enum Color; case red: return 1; case green: return 2; case blue: return 3; } return 0; }
int main() { return code(Color::blue) == 3 && static_cast<int>(Color::green) == 1 ? 0 : 1; }
