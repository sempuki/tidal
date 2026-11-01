// tidal-rules: CPT-18
// tidal-expect: reject
// tidal-error-clang: requires more than 1 template argument
// tidal-error-gcc: wrong number of template arguments
// tidal-error-edg: expected a "<"
template <class T, class U> concept Pair = true;
template <Pair T> struct Box {};
