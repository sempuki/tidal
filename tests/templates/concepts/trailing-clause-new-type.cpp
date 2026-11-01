// tidal-rules: CPT-13
// tidal-expect: reject
// tidal-error-clang: expected '\)'
// tidal-error-gcc: requires-clause on type-id
// tidal-error-edg: expected a "\)"
template <class T> void make() { auto p = new (int () requires true); }
