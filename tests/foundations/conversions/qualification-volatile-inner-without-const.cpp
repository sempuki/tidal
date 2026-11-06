// tidal-rules: CONV-9
// tidal-expect: reject
// tidal-error-clang: cannot initialize
// tidal-error-gcc: invalid conversion
// tidal-note: Gap review 2026-11-05 of the text phase 10 wrote. Adding volatile at an inner level without const at the level above is no implicit conversion.
int** r = nullptr;
volatile int** s = r;
