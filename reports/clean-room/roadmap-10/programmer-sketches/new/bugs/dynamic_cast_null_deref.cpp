// Planted bug: the result of a failed dynamic_cast is used without a check.
struct Shape { virtual ~Shape() = default; };
struct Circle : Shape { int r = 3; };
struct Square : Shape { int s = 4; };
int area(Shape* s) { return dynamic_cast<Circle*>(s)->r * 3; }
int main() {
  Square sq;
  return area(&sq) == 9 ? 0 : 1;
}
