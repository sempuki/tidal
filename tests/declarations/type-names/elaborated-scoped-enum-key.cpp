// tidal-rules: TYN-4
// tidal-expect: reject
// tidal-error-clang: must use .enum. not .enum class
// tidal-error-gcc: scoped enum must not use the .class. keyword
// tidal-error-edg: cannot be used here \(use plain .enum. instead\)
enum class E { a };
enum class E* p;
