// tidal-rules: CONV-8
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: with an rvalue of type .char.
// tidal-error-gcc: invalid conversion from .char. to .int\*.
// tidal-note: Q-0100. EDG accepts a character literal with value zero as a null pointer constant. Clang and GCC reject it.
// tidal-note: Gap review 2026-10-31 (third).
int* p = '\0';
