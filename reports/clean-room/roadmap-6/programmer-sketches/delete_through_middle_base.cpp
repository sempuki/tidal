struct B { virtual ~B() = default; };
struct M : B {};
struct D : M {};
int main() {
  M* p = new D;
  delete p;
  return 0;
}
