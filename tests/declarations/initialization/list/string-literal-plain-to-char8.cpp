// tidal-rules: LIST-6
// tidal-expect: reject
// tidal-error-clang: initializing .char8_t. array with plain string literal
// tidal-error-gcc: cannot initialize array of .char8_t. from a string literal
// tidal-error-edg: const char \[2\]. cannot be used to initialize an entity of type .char8_t
char8_t a[] = "x";
