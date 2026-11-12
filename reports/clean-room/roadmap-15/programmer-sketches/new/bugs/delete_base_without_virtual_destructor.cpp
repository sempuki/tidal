struct Shape { int id = 1; };
struct Circle : Shape { int r = 2; virtual ~Circle() {} };
int main() {
  Shape* s = new Circle;
  delete s;  // the static type has no virtual destructor
  return 0;
}
