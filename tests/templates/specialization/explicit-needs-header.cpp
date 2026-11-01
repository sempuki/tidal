// tidal-rules: TSP-4
// tidal-expect: reject
// tidal-error-clang: template specialization requires
// tidal-error-gcc: an explicit specialization must be preceded by
// tidal-error-edg: without .template<>. syntax
template <class T> struct Box {};
struct Box<int> {};
