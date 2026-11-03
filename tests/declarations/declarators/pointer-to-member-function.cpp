// tidal-rules: DCL-12
// tidal-expect: accept
// tidal-note: A pointer to a const member function with a parameter.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct S {
  int v;
  constexpr int add(int x) const { return v + x; }
};
constexpr int (S::* pf)(int) const = &S::add;
static_assert(same<decltype(&S::add), int (S::*)(int) const>);
static_assert((S{3}.*pf)(4) == 7);
