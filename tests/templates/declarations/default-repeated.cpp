// tidal-rules: TPL-16
// tidal-expect: reject
// tidal-error-clang: template parameter redefines default argument
// tidal-error-gcc: redefinition of default argument for
// tidal-error-edg: redefinition of default argument
// tidal-note: The default is the same in both declarations, and the repetition is still not valid.
template <class T = int> struct Box;
template <class T = int> struct Box {};
