// tidal-rules: CTAD-14 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide|requires template arguments
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor "Same" matches
// tidal-note: Same<T> is Pair<T, T>, and the arguments give int and char.
template <class T, class U> struct Pair { Pair(T, U) {} };
template <class T> using Same = Pair<T, T>;
Same s(1, 'c');
