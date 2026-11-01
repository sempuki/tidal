// tidal-rules: TARG-15
// tidal-expect: reject
// tidal-error-clang: pointer to subobject of string literal is not allowed in a
// tidal-error-gcc: because string literals can never be used in this context
// tidal-error-edg: invalid nontype template argument of type
template <const char* S> struct Name {};
Name<"text"> n;
