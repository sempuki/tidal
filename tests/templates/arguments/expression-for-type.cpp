// tidal-rules: TARG-7
// tidal-expect: reject
// tidal-error-clang: template argument for template type parameter must be a type
// tidal-error-gcc: type/value mismatch at argument 1 in template parameter
// tidal-error-edg: expected a type specifier
template <class T> struct Box {};
Box<1> b;
