// tidal-rules: CTAD-16 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor
// tidal-note: Gap review RV8-B-6. The base is an aggregate, so it has an aggregate candidate for {1}. Derived's own constructor template takes U and gives no guide that deduces T. If Derived took the base's aggregate candidate, it would deduce Derived<int>, and the constructor template would then initialize it. Every compiler rejects, so a derived class does not take the candidate. The review reads the inherited guides as the base's whole set, which holds the candidate. Clang makes no inherited guides (Q-0239).
template <class T> struct Base { T v; };
template <class T> struct Derived : Base<T> {
  using Base<T>::Base;
  template <class U> constexpr Derived(U) : Base<T>{} {}
};
Derived d{1};
