struct B { int x = 0; };
struct D : B { int y = 1; };
int main() {
  B* p = new D;
  delete p;
  return 0;
}
