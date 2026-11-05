struct B { virtual ~B() = default; };
struct M : B { ~M() {} };
struct D : M {};
int main() {
  M* p = new D;
  delete p;
  return 0;
}
