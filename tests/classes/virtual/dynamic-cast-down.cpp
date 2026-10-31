// tidal-rules: VIRT-10
// tidal-expect: run
struct B { virtual ~B() {} };
struct D : B { int v = 7; };
struct E : B {};
int main() { D d; E e; B* p = &d; B* q = &e;
  D* a = dynamic_cast<D*>(p); D* b = dynamic_cast<D*>(q);
  return a && a->v == 7 && b == nullptr ? 0 : 1; }
