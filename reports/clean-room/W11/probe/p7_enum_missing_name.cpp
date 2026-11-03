enum class Color : unsigned char { Red, Green, Blue, Yellow, Purple, Count };
constexpr const char* color_names[] = { "red", "green", "blue", "yellow" };
static_assert(sizeof color_names / sizeof color_names[0] == static_cast<int>(Color::Count), "one name per enumerator");
int main() { return 0; }
