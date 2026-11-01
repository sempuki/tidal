// tidal-rules: TSP-6
// tidal-expect: reject
// tidal-error-clang: after instantiation
// tidal-error-gcc: after instantiation
// tidal-error-edg: must precede its first use
// tidal-note: Box<int> is complete before the explicit specialization appears.
template <class T> struct Box { T v; };
Box<int> early;
template <> struct Box<int> { long v; };
