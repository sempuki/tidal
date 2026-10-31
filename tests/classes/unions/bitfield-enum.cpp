// tidal-rules: BF-1
// tidal-expect: run
enum class Color : unsigned { red, green, blue };
struct P { Color c : 2; };
int main() { P p{}; p.c = Color::blue; return p.c == Color::blue ? 0 : 1; }
