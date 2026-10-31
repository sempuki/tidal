// tidal-rules: ATTR-2
// tidal-expect: reject
// tidal-error-clang: nodiscard. attribute takes no more than 1 argument
// tidal-error-gcc: wrong number of arguments specified for .nodiscard
[[nodiscard("a", "b")]] int f();
