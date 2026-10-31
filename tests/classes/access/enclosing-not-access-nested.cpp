// tidal-rules: ACC-19
// tidal-expect: reject
// tidal-error-clang: is a private member
// tidal-error-gcc: (private|inaccessible) within this context
// tidal-error-edg: is inaccessible
class Outer {
  class Inner { int secret = 1; };
  int look(const Inner& i) { return i.secret; }
};
