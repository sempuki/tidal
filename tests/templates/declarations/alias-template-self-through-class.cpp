// tidal-rules: TPL-31
// tidal-expect: reject
// tidal-error-clang: no type named 'type' in
// tidal-error-gcc: invalid use of incomplete type
// tidal-error-edg: has no member "type"
// tidal-note: Self<int> needs Holder<int>::type, which needs Self<int>.
template <class T> struct Holder;
template <class T> using Self = typename Holder<T>::type;
template <class T> struct Holder { using type = Self<T>*; };
Self<int> x;
