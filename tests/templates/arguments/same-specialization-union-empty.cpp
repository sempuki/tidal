// tidal-rules: TARG-19
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: is not a constant expression
// tidal-note: A union with no active member is the same argument as another one. GCC does not accept the union value as a constant (Q-0202).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
union Choice {
  int i;
  char c;
  constexpr Choice() {}
};
template <Choice C> struct Pick {};
static_assert(same<Pick<Choice{}>, Pick<Choice{}>>);
