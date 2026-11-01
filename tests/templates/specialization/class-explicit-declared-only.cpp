// tidal-rules: TSP-20
// tidal-expect: reject
// tidal-error-clang: variable has incomplete type
// tidal-error-gcc: has incomplete type and cannot be defined
// tidal-error-edg: incomplete type
// tidal-note: The explicit specialization is declared and not defined, so the specialization is an incomplete type.
template <class T> struct Box { T v; };
template <> struct Box<int>;
Box<int> b;
