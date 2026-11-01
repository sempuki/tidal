// tidal-rules: STG-22
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=vptr -fno-sanitize-recover=all
// tidal-error: member call on address .* which does not point to an object of type
// tidal-note: Run-time check with the sanitizer (decision 0028).
struct Shape {
  virtual ~Shape() {}
  int sides() const { return 3; }
};
int main() {
  alignas(Shape) unsigned char buffer[sizeof(Shape)] = {};
  Shape* shape = reinterpret_cast<Shape*>(buffer);
  return shape->sides() == 3 ? 0 : 1;
}
