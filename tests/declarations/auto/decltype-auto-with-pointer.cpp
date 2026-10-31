// tidal-rules: AUTO-11
// tidal-expect: reject
// tidal-error-clang: cannot form pointer to .decltype\(auto\)
// tidal-error-gcc: as type rather than plain .decltype\(auto\)
// tidal-error-edg: must be a placeholder for the complete type
int i = 0;
decltype(auto)* p = &i;
