// tidal-rules: TSP-8
// tidal-expect: reject
// tidal-error-clang: redefinition of default argument
// tidal-error-gcc: default argument specified in explicit specialization
// tidal-error-edg: default argument is not allowed
struct Host { template <class T> int get(T, int = 0); };
template <> int Host::get(char, int = 1) { return 0; }
