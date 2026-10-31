// tidal-rules: TR-1 CONV-18
// tidal-expect: reject
// tidal-error-clang: cannot initialize a variable of type .int. with an lvalue of type .const char
// tidal-error-gcc: invalid conversion from .const char\*. to .int
// tidal-error-edg: const char \*. cannot be used to initialize an entity of type .int
int x = "text";
