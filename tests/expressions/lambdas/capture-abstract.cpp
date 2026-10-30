// tidal-rules: LAM-6
// tidal-expect: reject
struct Shape { virtual int area() = 0; };
void f(Shape& s) {
  auto g = [s] { return 1; };
}
