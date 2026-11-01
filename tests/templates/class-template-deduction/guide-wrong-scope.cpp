// tidal-rules: CTAD-12
// tidal-expect: reject
// tidal-error-clang: expected '\)'
// tidal-error-gcc: should have been declared inside
// tidal-error-edg: must be declared in the same scope
// tidal-note: The guide for lib::Box is declared outside lib.
namespace lib { template <class T> struct Box { Box(T) {} }; }
template <class T> lib::Box(T*) -> lib::Box<T>;
