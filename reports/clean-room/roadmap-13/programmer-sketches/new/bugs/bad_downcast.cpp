// A static_cast to the wrong derived class.
struct Shape { virtual ~Shape() = default; int id = 0; };
struct Circle : Shape { int r = 1; };
struct Square : Shape { int side = 2; };
int side_of(Shape& s) { return static_cast<Square&>(s).side; }
int main() { Circle c; return side_of(c) == 2 ? 0 : 1; }
