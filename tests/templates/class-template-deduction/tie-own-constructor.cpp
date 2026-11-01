// tidal-rules: CTAD-19
// tidal-expect: accept
// tidal-note: Both guides come from a constructor that is not a template and take T. The guide from the constructor of Derived wins over the inherited explicit one, so copy-list initialization works. Clang has no inherited guides (Q-0239) and gives the same result.
template <class T> struct Base { explicit constexpr Base(T) {} };
template <class T> struct Derived : Base<T> { using Base<T>::Base; constexpr Derived(T x) : Base<T>(x) {} };
Derived d = {1};
