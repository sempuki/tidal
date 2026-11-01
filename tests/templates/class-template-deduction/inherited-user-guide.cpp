// tidal-rules: CTAD-16
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-note: The user guide of Base reaches Derived through the inherited constructors (Q-0239).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Base { template <class U> constexpr Base(U) {} };
template <class T> Base(T*) -> Base<T>;
template <class T> struct Derived : Base<T> { using Base<T>::Base; };
int n = 0;
Derived d(&n);
static_assert(same<decltype(d), Derived<int>>);
