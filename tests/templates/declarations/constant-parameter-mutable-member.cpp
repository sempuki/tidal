// tidal-rules: TPL-7
// tidal-expect: reject
// tidal-error-clang: of non-type template parameter is not a structural type
// tidal-error-gcc: is not a valid type for a template non-type parameter
// tidal-error-edg: a template parameter of class type must be of structural
struct Counter { mutable int hits = 0; };
template <Counter C> struct Use {};
