// tidal-rules: TSP-27
// tidal-expect: reject
// tidal-error-clang: does not specialize an instantiated member
// tidal-error-gcc: does not match any template declaration
// tidal-error-edg: is not an entity that can be explicitly specialized
// tidal-note: The destructor of Box<int> is declared implicitly.
template <class T> struct Box { T value; };
template <> Box<int>::~Box() {}
