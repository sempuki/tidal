// tidal-rules: CPT-36
// tidal-expect: reject
// tidal-error-clang: too many template arguments for concept 'Any'
// tidal-error-gcc: wrong number of template arguments
// tidal-error-edg: too many arguments for concept "Any"
template <class T> concept Any = true;
static_assert(!Any<int, int>);
