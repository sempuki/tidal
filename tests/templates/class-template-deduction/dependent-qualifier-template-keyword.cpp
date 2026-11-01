// tidal-rules: CTAD-1
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: (template argument list is expected after a name prefixed by the template keyword|expected an identifier or template-id)
// tidal-note: The keyword template comes before the deduced name In after a dependent qualifier. GCC and EDG deduce In<int>, and Clang asks for an argument list (Q-0243).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Outer { template <class U> struct In { constexpr In(U) {} }; };
template <class T> constexpr bool check() {
  typename Outer<T>::template In x(1);
  return same<decltype(x), typename Outer<T>::template In<int>>;
}
static_assert(check<char>());
