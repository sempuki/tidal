// tidal-rules: ODR-5 ODR-8
// tidal-expect: run
// tidal-note: The definition of the destructor uses Shape::operator delete, which has no definition, so the program is not valid. Clang and GCC emit no reference to the function for a destructor that is not virtual, and the program links (ODR-8 needs no diagnostic).
struct Shape {
  ~Shape() {}
  static void operator delete(void*);
};
int main() { Shape shape; return 0; }
