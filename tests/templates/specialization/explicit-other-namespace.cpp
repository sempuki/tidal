// tidal-rules: TSP-5
// tidal-expect: reject
// tidal-error-clang: not in a namespace enclosing
// tidal-error-gcc: which does not enclose
// tidal-error-edg: cannot be specialized in the current scope
// tidal-note: The namespace other does not enclose the namespace of the template.
namespace lib { template <class T> struct Box {}; }
namespace other { template <> struct lib::Box<int> {}; }
