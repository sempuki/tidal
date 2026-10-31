// tidal-rules: CONV-9
// tidal-expect: reject
// tidal-error-clang: type .const int \*\*. with an lvalue of type .int \*\*.
// tidal-error-gcc: invalid conversion from .int\*\*. to .const int\*\*.
// tidal-error-edg: int \*\*. cannot be used to initialize an entity of type .const int \*\*
// tidal-note: Gap review 2026-10-31 (third).
int** pp = nullptr;
const int** q = pp;
