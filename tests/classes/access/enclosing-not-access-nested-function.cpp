// tidal-rules: ACC-19
// tidal-expect: reject
// tidal-error-clang: is a private member
// tidal-error-gcc: (private|inaccessible) within this context
// tidal-error-edg: is inaccessible
// tidal-note: The enclosing class calls a private static member function of the nested class.
class Outer {
  class Inner {
    static int make() { return 1; }
  };
  int g() { return Inner::make(); }
};
