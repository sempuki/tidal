// tidal-rules: NEW-8
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-note: Q-0035. GCC accepts delete of a void pointer with a warning that it is undefined; Clang (pedantic) and EDG reject it.
void f(void* p) { delete p; }
