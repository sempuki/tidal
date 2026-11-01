// tidal-rules: STG-18 STG-22
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=vptr -fno-sanitize-recover=all
// tidal-error: member call on address .* which does not point to an object of type
// tidal-note: A class with a virtual function is not an implicit-lifetime type, so no Shape exists in the storage.
// tidal-note: Run-time check with the sanitizer (decision 0028).
struct Shape {
  virtual ~Shape() {}
  int sides() const { return 3; }
};
int main() {
  void* raw = ::operator new(sizeof(Shape));
  Shape* shape = static_cast<Shape*>(raw);
  int sides = shape->sides();
  ::operator delete(raw);
  return sides == 3 ? 0 : 1;
}
