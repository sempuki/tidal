// tidal-rules: CTAD-19 CTAD-23
// tidal-expect: reject
// tidal-error-clang: selected an explicit constructor for copy-list-initialization
// tidal-error-gcc: explicit deduction guide selected in copy-list-initialization
// tidal-error-edg: copy-list-initialization cannot use "explicit"
// tidal-note: The own explicit constructor template wins over the inherited constructor that is not a template, so CTAD-19 comes before CTAD-22.
template <class T> struct Base { Base(T) {} };
template <class T> struct Derived : Base<T> {
  using Base<T>::Base;
  template <class U = int> explicit Derived(T x, U* = nullptr) : Base<T>(x) {}
};
Derived d = {1};
