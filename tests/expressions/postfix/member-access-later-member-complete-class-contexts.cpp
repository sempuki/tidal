// tidal-rules: PFX-11
// tidal-expect: accept
// tidal-note: Gap review of the text phase 8 wrote on expressions. A default member initializer, a default argument, and the body of a member function of a nested class each see every member of the enclosing class, the later ones included. Clang 22, GCC 16, and EDG 7.0 accept.
struct Shape {
  int size = sizeof(this->later);
  int width(int x = sizeof(static_cast<Shape*>(nullptr)->later)) { return x; }
  struct Inner { int get(Shape& s) { return s.later; } };
  int later = 2;
};
