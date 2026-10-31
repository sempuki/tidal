// tidal-rules: TYN-6
// tidal-expect: run
enum class Color : unsigned char;
unsigned char code(Color c) { return static_cast<unsigned char>(c); }
enum class Color : unsigned char { red = 7 };
int main() { return code(Color::red) == 7 ? 0 : 1; }
