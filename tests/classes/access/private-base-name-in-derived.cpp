// tidal-rules: ACC-8
// tidal-expect: reject
struct A { int v; };
class B : private A {};
class C : public B {
  int get() { return v; }
};
