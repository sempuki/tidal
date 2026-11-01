// tidal-rules: TPL-7
// tidal-expect: reject
// tidal-error-clang: non-type template parameter has non-literal type
// tidal-error-gcc: is not a valid type for a template non-type parameter
// tidal-error-edg: a template parameter of class type must be of structural
struct Owner { int v; ~Owner() {} };
template <Owner O> struct Use {};
