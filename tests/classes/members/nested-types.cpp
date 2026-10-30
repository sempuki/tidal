// tidal-rules: CLS-8
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
struct Outer {
  struct Inner { int v; };
  using Size = unsigned;
  enum Kind { A, B };
};
Outer::Inner i{1};
static_assert(same<Outer::Size, unsigned>);
static_assert(Outer::B == 1);
