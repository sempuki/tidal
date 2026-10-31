// tidal-rules: CONV-9
// tidal-expect: reject
// tidal-error-clang: type .void \*. with an rvalue of type .const int \*.
// tidal-error-gcc: invalid conversion from .const void\*. to .void\*.
// tidal-error-edg: const int \*. cannot be used to initialize an entity of type .void \*
const int c = 0;
void* v = &c;
