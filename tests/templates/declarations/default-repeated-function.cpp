// tidal-rules: TPL-16
// tidal-expect: reject
// tidal-error-clang: template parameter redefines default argument
// tidal-error-gcc: redefinition of default argument for
// tidal-error-edg: redefinition of default argument
// tidal-note: Two declarations of the function template give the same default for T.
template <class T = int> void fill();
template <class T = int> void fill() {}
