// tidal-rules: CONV-20
// tidal-expect: run
struct Base { int tag = 1; };
struct Derived : Base { int extra = 2; };
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;

constexpr int take(int v = 2.7) { return v; }
static_assert(take() == 2);

constexpr bool flag = true;
static_assert(same<decltype(flag ? 1 : 2.5), double>);
static_assert((flag ? 1 : 2.5) == 1.0);

constexpr int add() {
  int v = 1;
  v += 2.7;
  return v;
}
static_assert(add() == 3);

constexpr unsigned char wrap() {
  unsigned char c = 250;
  c += 10;
  return c;
}
static_assert(wrap() == 4);

int caught() {
  Derived d;
  try {
    throw &d;
  } catch (Base* b) {
    return b->tag;
  }
  return 0;
}

int main() {
  if (take() != 2) return 1;
  if (add() != 3) return 2;
  if (wrap() != 4) return 3;
  if (caught() != 1) return 4;
  return 0;
}
