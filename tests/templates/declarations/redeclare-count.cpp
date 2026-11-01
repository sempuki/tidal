// tidal-rules: TPL-20
// tidal-expect: reject
// tidal-error-clang: too many template parameters in template redeclaration
// tidal-error-gcc: redeclared with 2 template parameters
// tidal-error-edg: too many template parameters -- does not match previous
template <class T> struct Box;
template <class T, class U> struct Box {};
