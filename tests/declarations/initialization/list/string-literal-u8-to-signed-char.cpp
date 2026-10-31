// tidal-rules: LIST-6
// tidal-expect: reject
// tidal-error-clang: char array with UTF-8 string literal is not permitted
// tidal-error-gcc: cannot initialize array of .signed char. from a string literal
// tidal-error-edg: const char8_t \[2\]. cannot be used to initialize an entity of type .signed char
signed char a[] = u8"x";
