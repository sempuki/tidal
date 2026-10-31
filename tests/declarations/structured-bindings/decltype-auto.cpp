// tidal-rules: SB-7
// tidal-expect: reject
// tidal-error-clang: with type .decltype\(auto\).; declared type must be .auto
// tidal-error-gcc: cannot have type .decltype\(auto\)
// tidal-error-edg: invalid specifier for structured binding declaration
int a[2];
decltype(auto) [x, y] = a;
