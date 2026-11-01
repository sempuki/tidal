// tidal-rules: CPT-34
// tidal-expect: reject
// tidal-error-clang: expected concept name with optional arguments
// tidal-error-gcc: return-type-requirement is not a type-constraint
// tidal-error-edg: expected a concept name
template <class T> concept IsInt = requires (T t) { { t } -> int; };
