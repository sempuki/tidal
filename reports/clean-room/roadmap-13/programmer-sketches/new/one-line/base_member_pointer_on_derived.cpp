// A pointer to a member of a base, applied to a derived object.
struct Base { int id = 3; int get() const { return id; } };
struct Derived : Base { int more = 4; };
int main() {
  int Base::*pm = &Base::id;
  int (Base::*pf)() const = &Base::get;
  Derived d;
  d.*pm = 9;
  return (d.*pf)() - 9;
}
