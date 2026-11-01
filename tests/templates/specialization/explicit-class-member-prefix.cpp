// tidal-rules: TSP-23
// tidal-expect: reject
// tidal-error-clang: no function template matches function template specialization
// tidal-error-gcc: does not match any template declaration
// tidal-error-edg: is not an entity that can be explicitly specialized
template <class T> struct Box { int get(); };
template <> struct Box<int> { int get(); };
template <> int Box<int>::get() { return 1; }
