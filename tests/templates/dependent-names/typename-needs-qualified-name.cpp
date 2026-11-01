// tidal-rules: DEP-21 DEP-29
// tidal-expect: reject
// tidal-error-clang: expected a qualified name after 'typename'
// tidal-error-gcc: expected nested-name-specifier
// tidal-error-edg: qualified name is required
template <class T> void make() { typename T value; }
