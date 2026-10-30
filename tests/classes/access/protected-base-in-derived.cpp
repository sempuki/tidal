// tidal-rules: ACC-8
// tidal-expect: accept
struct B { int x; };
class D : protected B {
public:
  int get() { return x; }
};
