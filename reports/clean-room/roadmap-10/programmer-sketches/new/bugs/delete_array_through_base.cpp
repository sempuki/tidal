// Planted bug: delete[] of an array of Derived through a pointer to Base.
struct Base { virtual ~Base() = default; int a = 0; };
struct Derived : Base { int b = 1; };
int main() {
  Base* p = new Derived[3];
  delete[] p;
  return 0;
}
