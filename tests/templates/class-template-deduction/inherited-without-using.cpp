// tidal-rules: CTAD-16 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor "Derived" matches
// tidal-note: Derived does not inherit the constructors of Base, so Derived has no guide that takes an int.
template <class T> struct Base { Base(T) {} };
template <class T> struct Derived : Base<T> {};
Derived d(1);
