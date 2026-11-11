// Inherited constructors with using Base::Base, and a derived member default.
struct Base {
  int a, b;
  constexpr Base(int x) : a(x), b(0) {}
  constexpr Base(int x, int y) : a(x), b(y) {}
};
struct Derived : Base {
  using Base::Base;
  int extra = 5;
};
static_assert(Derived(1, 2).b == 2 && Derived(3).extra == 5);
int main() { Derived d(4); return d.a + d.extra - 9; }
