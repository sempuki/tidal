// tidal-rules: VIRT-17
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to .A::~A\(\)
// tidal-error-gcc: undefined reference to .A::~A\(\)
// tidal-note: The derived object is created by new and deleted through a pointer to the base.
struct A { virtual ~A() = 0; };
struct B : A { int v = 1; };
int main() {
  A* p = new B;
  delete p;
  return 0;
}
