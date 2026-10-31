// tidal-rules: RET-1
// tidal-expect: reject
// tidal-error-clang: deduced as .int. here but deduced as .void. in earlier return statement
// tidal-error-gcc: inconsistent deduction for auto return type
// tidal-error-edg: conflicts with previously deduced type
auto f(bool c) { if (c) return; return 1; }
