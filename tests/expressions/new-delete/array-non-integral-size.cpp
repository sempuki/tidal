// tidal-rules: NEW-2
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: must have integral or enumeration type
// tidal-note: Q-0036. Clang and EDG convert a floating array size in a new expression to an integer; GCC rejects it.
void f() { int* p = new int[1.5]; }
