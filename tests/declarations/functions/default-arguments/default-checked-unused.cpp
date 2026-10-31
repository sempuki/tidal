// tidal-rules: DEFARG-5
// tidal-expect: reject
// tidal-error-clang: cannot initialize a parameter of type .int \*. with an rvalue of type .double
// tidal-error-gcc: could not convert .* from .double. to .int\*
// tidal-error-edg: default argument of type .double. is incompatible
// tidal-note: Gap review 2026-10-31.
void f(int* p = 1.5);
