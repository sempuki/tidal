// tidal-rules: CTAD-10 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor "Agg" matches
// tidal-note: The second element has no member, so no aggregate candidate is formed.
template <class T> struct Agg { T v; };
Agg a{1, 2};
