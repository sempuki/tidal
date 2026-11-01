// tidal-rules: CTAD-8 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor "Wrap" matches
// tidal-note: T appears only before :: in the parameter type, which is a non-deduced position.
template <class T> struct Id { using type = T; };
template <class T> struct Wrap { Wrap(typename Id<T>::type) {} };
Wrap w(1);
