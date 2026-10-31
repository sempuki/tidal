// tidal-rules: ACC-18
// tidal-expect: reject
// tidal-error-clang: is a private member
// tidal-error-gcc: (private|inaccessible) within this context
// tidal-error-edg: is inaccessible
struct A { int v; };
class B : private A {};
class C : public B {
  int get() { return v; }
};
