// tidal-rules: CTAD-16 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor "Derived" matches
// tidal-note: The base Base<int> does not name T, so the inherited guide gives T no value.
template <class T> struct Base { Base(T) {} };
template <class T> struct Derived : Base<int> { using Base<int>::Base; };
Derived d(1);
