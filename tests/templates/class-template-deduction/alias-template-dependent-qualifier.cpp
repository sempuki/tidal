// tidal-rules: CTAD-15
// tidal-expect: reject
// tidal-error-clang: requires template arguments; argument deduction only allowed for class templates
// tidal-error-gcc: invalid use of template-name
// tidal-error-edg: no viable template argument deduction candidate
// tidal-note: The alias names the class template In through the dependent qualifier Holder<T>::, so the alias cannot stand without arguments.
template <class T> struct Holder { template <class U> struct In { constexpr In(U) {} }; };
template <class T> using A = typename Holder<T>::template In<T>;
A a(1);
