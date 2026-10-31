// tidal-rules: CE-24 CINIT-1
// tidal-expect: reject
// tidal-error-clang: does not have a constant initializer
// tidal-error-gcc: does not have a constant initializer
// tidal-error-edg: cannot call non-constexpr function
int read_setting();
constinit int setting = read_setting();
