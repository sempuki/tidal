// tidal-rules: CPT-36
// tidal-expect: reject
// tidal-error-clang: template argument for template type parameter must be a type
// tidal-error-gcc: type/value mismatch
// tidal-error-edg: expected a type specifier
template <class T> concept Any = true;
static_assert(!Any<1>);
