// tidal-rules: VIRT-10
// tidal-expect: run
struct A { virtual ~A() {} };
struct B { virtual ~B() {} };
struct D : public A, private B { B* hidden() { return this; } };
int main() {
  D d;
  bool cross_fails = dynamic_cast<A*>(d.hidden()) == nullptr;
  bool down_fails = dynamic_cast<D*>(d.hidden()) == nullptr;
  return cross_fails && down_fails ? 0 : 1;
}
