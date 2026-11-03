// tidal-rules: UN-10
// tidal-expect: accept
// tidal-note: Prefix -- on a pointer and on a double. The result is the operand itself.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
constexpr bool check() {
  int values[3] = {1, 2, 3};
  int* p = values + 2;
  double d = 1.5;
  int** pp = &p;
  bool same_object = &--p == pp;
  --d;
  return same_object && *p == 2 && d == 0.5;
}
static_assert(check());
double x;
static_assert(same<decltype((--x)), double&>);
