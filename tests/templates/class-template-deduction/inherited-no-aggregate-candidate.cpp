// tidal-rules: CTAD-16 CTAD-9 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor
// tidal-note: The base is an aggregate and has an aggregate candidate for {1}. A derived class does not take that candidate, and Derived is not an aggregate (DER-8), so no guide gives T.
template <class T> struct Base { T v; };
template <class T> struct Derived : Base<T> { using Base<T>::Base; };
Derived d{1};
