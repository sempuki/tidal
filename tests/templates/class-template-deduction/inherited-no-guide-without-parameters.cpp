// tidal-rules: CTAD-16 CTAD-6 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-note: Derived has its own guide with no parameters (CTAD-6), and its T has no default, so that guide gives no T. The guide with no parameters of Base, which would give T = int, is not one of the guides that Derived takes.
// tidal-note: The base declares no constructor, so the base has the guide with no parameters (CTAD-6). A derived class does not take that guide. Derived declares no constructor of its own either, but its using-declaration does not give it a guide that deduces T from the base's default, so nothing gives T.
template <class T = int> struct Base {};
template <class T> struct Derived : Base<T> { using Base<T>::Base; };
Derived d;
