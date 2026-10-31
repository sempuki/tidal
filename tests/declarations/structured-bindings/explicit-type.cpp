// tidal-rules: SB-15
// tidal-expect: reject
// tidal-error-clang: with type .int.; declared type must be .auto
// tidal-error-gcc: structured binding declaration cannot have type .int
// tidal-error-edg: invalid specifier for structured binding declaration
int a[2];
int [x, y] = a;
