// tidal-rules: PACK-11
// tidal-expect: reject
// tidal-error-clang: pack expansion used as argument for non-pack parameter of alias template
// tidal-error-gcc: pack expansion argument for non-pack parameter
// tidal-error-edg: a pack expansion cannot be used as an argument
template <class A, class B> using Second = B;
template <class... T> struct Hold { using type = Second<T...>; };
