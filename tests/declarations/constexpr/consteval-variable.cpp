// tidal-rules: CEV-3
// tidal-expect: reject
// tidal-error-clang: consteval can only be used in function declarations
// tidal-error-gcc: variable cannot be declared .consteval
// tidal-error-edg: consteval. is not permitted on the declaration of a variable
consteval int n = 1;
