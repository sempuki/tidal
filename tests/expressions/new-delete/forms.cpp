// tidal-rules: NEW-1
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
constexpr bool check() {
  int* a = new int(5);
  int* b = new int{6};
  int* c = new int();
  auto* d = new auto(7);
  auto* e = new auto{8};
  static_assert(same<decltype(d), int*>);
  bool ok = *a == 5 && *b == 6 && *c == 0 && *d == 7 && *e == 8;
  delete a; delete b; delete c; delete d; delete e;
  return ok;
}
static_assert(check());
