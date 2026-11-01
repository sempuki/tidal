// tidal-rules: CTAD-1
// tidal-expect: accept
// tidal-note: The qualifier Outer<T>:: is dependent. Every compiler deduces In<int> after instantiation (Q-0243).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Outer { template <class U> struct In { constexpr In(U) {} }; };
template <class T> constexpr bool check() {
  typename Outer<T>::In x(1);
  return same<decltype(x), typename Outer<T>::template In<int>>;
}
static_assert(check<char>());
