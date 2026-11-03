// tidal-rules: INIT-3 DCL-13
// tidal-expect: accept
// tidal-note: Write report W6. Tracker(&live) has the form of a parameter, so h is a function. Braces or an extra pair of parentheses make a variable.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Tracker {
  int* count;
  constexpr Tracker(int* c) : count(c) { ++*count; }
};
struct Holder { const Tracker& ref; };
constexpr int made() {
  int live = 0;
  Holder h(Tracker(&live));
  static_assert(same<decltype(h), Holder(Tracker&)>);
  Holder braced(Tracker{&live});
  Holder doubled((Tracker(&live)));
  (void)braced;
  (void)doubled;
  return live;
}
static_assert(made() == 2);
