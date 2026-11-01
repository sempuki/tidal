// tidal-rules: TARG-10
// tidal-expect: reject
// tidal-error-clang: requires template arguments
// tidal-error-gcc: missing template arguments before
// tidal-error-edg: argument list for variable template
template <class T> constexpr int width = sizeof(T);
int w = width;
