// tidal-rules: CTAD-16
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-note: Derived inherits the constructors of Base<T>, so the guide from Base(T) reaches Derived. Clang has no guides from inherited constructors (Q-0239).
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Base { constexpr Base(T) {} };
template <class T> struct Derived : Base<T> { using Base<T>::Base; };
Derived d(1);
static_assert(same<decltype(d), Derived<int>>);
