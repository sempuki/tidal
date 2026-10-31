// tidal-rules: LIST-15
// tidal-expect: reject
// tidal-error-clang: initializer-string for char array is too long
// tidal-error-gcc: initializer-string for .char \[2\]. is too long
// tidal-error-edg: const char \[3\]. cannot be used to initialize an entity of type .char \[2\]
char a[2] = "ab";
