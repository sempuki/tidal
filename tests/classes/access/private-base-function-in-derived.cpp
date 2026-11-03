// tidal-rules: ACC-18
// tidal-expect: reject
// tidal-error-clang: is a private member
// tidal-error-gcc: (private|inaccessible) within this context
// tidal-error-edg: is inaccessible
// tidal-note: The base is public, and the derived class calls a private member function of the base.
class A {
  int f() { return 1; }
};
class B : public A {
  int g() { return f(); }
};
