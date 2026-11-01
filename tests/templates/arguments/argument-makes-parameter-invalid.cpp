// tidal-rules: TARG-24
// tidal-expect: reject
// tidal-error-clang: cannot be used prior to '::' because it has no members
// tidal-error-gcc: is not a class, struct, or union type
// tidal-error-edg: must be a class or namespace name
// tidal-note: The type of N needs int::size_type.
template <class T, typename T::size_type N> struct Sized {};
Sized<int, 3> s;
