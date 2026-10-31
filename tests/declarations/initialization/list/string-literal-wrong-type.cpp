// tidal-rules: LIST-15
// tidal-expect: reject
// tidal-error-clang: initializing char array with wide string literal
// tidal-error-gcc: cannot initialize array of .char. from a string literal
// tidal-error-edg: const char16_t \[2\]. cannot be used to initialize an entity of type .char
char a[] = u"x";
