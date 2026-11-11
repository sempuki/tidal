// A default argument of a virtual function comes from the static type of the call.
struct Base { virtual int scale(int f = 2) const { return f; } virtual ~Base() = default; };
struct Derived : Base { int scale(int f = 10) const override { return f * 100; } };
int main() {
  Derived d;
  const Base& b = d;
  return b.scale() == 200 && d.scale() == 1000 ? 0 : 1;
}
