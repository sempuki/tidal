// tidal-rules: CONV-8
// tidal-expect: reject
// tidal-error-clang: with an rvalue of type .bool.
// tidal-error-gcc: cannot convert .bool. to .int\*.
// tidal-error-edg: bool. cannot be used to initialize an entity of type .int \*
// tidal-note: Gap review 2026-10-30.
int* p = false;
