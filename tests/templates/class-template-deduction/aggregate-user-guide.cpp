// tidal-rules: CTAD-10 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: class template argument deduction failed
// tidal-error-edg: no instance of constructor "Agg" matches
// tidal-note: The user guide removes the aggregate candidate, and the guide needs a pointer.
template <class T> struct Agg { T v; };
template <class T> Agg(T*) -> Agg<T>;
Agg a{1};
