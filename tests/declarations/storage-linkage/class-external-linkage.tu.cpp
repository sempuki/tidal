struct Point { int x; };
enum class Color { red, green };
int read(Point p) { return p.x; }
int pick(Color c) { return c == Color::green ? 1 : 0; }
