// tidal-rules: PACK-15
// tidal-expect: reject
// tidal-error-clang: pack expansion for initialization of member
// tidal-error-gcc: cannot expand initializer for member
// tidal-error-edg: a mem-initializer for a data member cannot be a pack
// tidal-note: The braced initializer of the data member value is expanded with ellipsis after it.
template <class... T> struct Hold { int value; Hold(T... x) : value{x}... {} };
Hold<int> h(1);
