// tidal-rules: CTAD-10 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: cannot deduce class template arguments
// tidal-note: The base Base<T> has a dependent type, so its braces are not elided. The aggregate candidate takes 1 for the base and 2 for w, and Base<T> cannot be deduced from int.
template <class T> struct Base { T v; };
template <class T> struct Wrap : Base<T> { T w; };
Wrap y{1, 2};
