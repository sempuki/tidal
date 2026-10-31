// tidal-rules: LAM-22
// tidal-expect: reject
// tidal-error-clang: must match previous return type
// tidal-error-gcc: inconsistent types .int. and .double. deduced
// tidal-error-edg: conflicts with previously deduced type
auto g = [](bool b) { if (b) return 1; return 2.0; };
