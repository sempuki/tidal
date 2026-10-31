// tidal-rules: REF-13
// tidal-expect: run
struct Base { int v = 1; };
struct Derived : Base {};
int main() {
  int x = 1;
  const int& same = x;
  const volatile int& more = x;
  Derived d;
  const Base& base = d;
  x = 2;
  d.v = 3;
  return &same == &x && &more == &x && same == 2 && &base == &d && base.v == 3 ? 0 : 1;
}
