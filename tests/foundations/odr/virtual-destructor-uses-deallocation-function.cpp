// tidal-rules: ODR-5 ODR-8
// tidal-expect: link-fail
// tidal-expect-gcc-16-o2: run
// tidal-expect-clang-22-o2: run
// tidal-error-clang: undefined reference to .Shape::operator
// tidal-error-gcc: undefined reference to .Shape::operator
// tidal-note: At -O2, GCC and Clang emit no deleting destructor for Shape, because no code deletes a Shape and the virtual table is not needed, so the program links (ODR-8 needs no diagnostic).
struct Shape {
  virtual ~Shape() {}
  static void operator delete(void*);
};
int main() { Shape shape; return 0; }
