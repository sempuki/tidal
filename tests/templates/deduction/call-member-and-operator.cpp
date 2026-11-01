// tidal-rules: DED-1
// tidal-expect: accept
// tidal-note: Deduction finds the template arguments for a member function template, a static member template named with its class, and an operator template.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Tag {};
struct Box {
  template <class T> Tag<T> get(T) const;
  template <class T> static Tag<T*> point(T*);
};
template <class T> Tag<T> operator+(Box, T);
int n = 0;
static_assert(same<decltype(Box{}.get(2.0)), Tag<double>>);
static_assert(same<decltype(Box::point(&n)), Tag<int*>>);
static_assert(same<decltype(Box{} + 'c'), Tag<char>>);
