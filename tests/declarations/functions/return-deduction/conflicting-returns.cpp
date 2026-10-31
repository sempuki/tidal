// tidal-rules: RET-7
// tidal-expect: reject
// tidal-error-clang: deduced as .double. here but deduced as .int. in earlier return statement
// tidal-error-gcc: inconsistent deduction for auto return type
// tidal-error-edg: conflicts with previously deduced type
auto f(bool c) { if (c) return 1; return 2.0; }
