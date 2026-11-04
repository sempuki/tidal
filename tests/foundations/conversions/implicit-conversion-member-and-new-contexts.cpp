// tidal-rules: CONV-20
// tidal-expect: run
// tidal-note: X17-4. A member initializer, a default member initializer, the initializer of new, the size of an array new, a reference to a temporary, the standard conversion before a converting constructor, and the step of ++ on a type narrower than int each convert implicitly.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;

struct Members {
  int fromDefault = 2.7;
  int fromInit;
  constexpr Members() : fromInit(3.9) {}
};
static_assert(Members().fromDefault == 2);
static_assert(Members().fromInit == 3);

struct Wrapper {
  int value;
  constexpr Wrapper(int v) : value(v) {}
};

constexpr int wrapped() {
  Wrapper w = 4.5;
  return w.value;
}
static_assert(wrapped() == 4);

constexpr int bound() {
  const int& r = 5.8;
  return r;
}
static_assert(bound() == 5);

constexpr unsigned char step() {
  unsigned char c = 255;
  ++c;
  return c;
}
static_assert(step() == 0);

constexpr int allocated() {
  int* p = new int(6.6);
  int v = *p;
  delete p;
  return v;
}
static_assert(allocated() == 6);

constexpr int sized() {
  short n = 3;
  int* p = new int[n]{1, 2, 3};
  int v = p[2];
  delete[] p;
  return v;
}
static_assert(sized() == 3);

int main() {
  if (Members().fromDefault != 2) return 1;
  if (wrapped() != 4) return 2;
  if (bound() != 5) return 3;
  if (step() != 0) return 4;
  if (allocated() != 6) return 5;
  if (sized() != 3) return 6;
  return 0;
}
