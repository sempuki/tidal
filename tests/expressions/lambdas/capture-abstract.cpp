// tidal-rules: LAM-6
// tidal-expect: reject
// tidal-error-clang: capture of value of abstract type
// tidal-error-gcc: abstract type .Shape.
// tidal-error-edg: abstract class type .Shape. is not allowed
struct Shape { virtual int area() = 0; };
void f(Shape& s) {
  auto g = [s] { return 1; };
}
