// tidal-rules: TARG-15
// tidal-expect: reject
// tidal-error-clang: predefined '__func__' variable is not allowed in a template argument
// tidal-error-gcc: is not a valid template argument
// tidal-error-edg: a template argument may not reference
template <const char* P> struct Name {};
void use() { Name<__func__> n; }
