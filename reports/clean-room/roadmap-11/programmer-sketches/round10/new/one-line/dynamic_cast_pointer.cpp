// dynamic_cast to a pointer: a successful downcast and a null result for the wrong type.
struct Shape { virtual ~Shape() = default; };
struct Circle : Shape { int r = 3; };
struct Square : Shape { int s = 4; };
int main() {
  Circle c;
  Shape* s = &c;
  Circle* ok = dynamic_cast<Circle*>(s);
  Square* no = dynamic_cast<Square*>(s);
  return ok && ok->r == 3 && no == nullptr ? 0 : 1;
}
