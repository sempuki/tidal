// tidal-rules: LIST-15
// tidal-expect: reject
// tidal-error-clang: initializing wide char array with incompatible wide string literal
// tidal-error-gcc: cannot initialize array of .char32_t. from a string literal
// tidal-error-edg: cannot be used to initialize an entity of type .char32_t
char32_t a[] = L"x";
