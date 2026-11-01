// tidal-rules: CTAD-16 CTAD-26
// tidal-expect: reject
// tidal-error-clang: no matching constructor for initialization of .Derived<int>.
// tidal-error-gcc: no matching function for call to .Derived<int>::Derived
// tidal-error-edg: no instance of constructor "Derived<T>::Derived \[with T=int\]"
// tidal-note: Every compiler deduces Derived<int> from an object of Base<int>, as the inherited copy candidate gives. The inherited constructors do not include the copy constructor of Base, so the initialization fails.
template <class T> struct Base { constexpr Base(T) {} };
template <class T> struct Derived : Base<T> { using Base<T>::Base; };
constexpr Base<int> b(1);
Derived d(b);
