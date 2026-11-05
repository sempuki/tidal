struct B { virtual ~B() = default; };
struct M : B {};
struct D : M {};
int main() {
  B* p = new D;
  delete p;
  return 0;
}
