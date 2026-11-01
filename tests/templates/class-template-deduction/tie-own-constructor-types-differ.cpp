// tidal-rules: CTAD-19 CTAD-24
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: (class template argument deduction failed|is ambiguous)
// tidal-error-edg: more than one instance of constructor "Derived" matches
// tidal-note: The guide from Derived(const T&) and the inherited guide from Base(T) tie, and their parameter types differ, so CTAD-19 does not decide. Clang makes no inherited guides and deduces Derived<int> (Q-0239).
template <class T> struct Base { constexpr Base(T) {} };
template <class T> struct Derived : Base<T> {
  using Base<T>::Base;
  constexpr Derived(const T&) : Base<T>(0) {}
};
Derived d(1);
