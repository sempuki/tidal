// tidal-rules: CTAD-19 CTAD-23
// tidal-expect: reject
// tidal-error-clang: selected an explicit constructor for copy-list-initialization
// tidal-error-gcc: explicit deduction guide selected in copy-list-initialization
// tidal-error-edg: copy-list-initialization cannot use "explicit"
// tidal-note: The own explicit constructor wins over the user guide that Derived inherits from Base, so CTAD-19 comes before CTAD-20.
template <class T> struct Base { template <class U> Base(U) {} };
template <class T> Base(T) -> Base<T>;
template <class T> struct Derived : Base<T> { using Base<T>::Base; explicit Derived(T x) : Base<T>(x) {} };
Derived d = {1};
