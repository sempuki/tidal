// tidal-rules: VIRT-10
// tidal-expect: run
struct A { virtual ~A() {} };
struct M : A {};
struct D : private M {
  A* inner() { return this; }
  M* middle() { return this; }
};
int main() { D d; return dynamic_cast<M*>(d.inner()) == d.middle() ? 0 : 1; }
