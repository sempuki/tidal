// tidal-rules: CONV-9
// tidal-expect: accept
// tidal-note: Gap review 2026-11-05 of the text phase 10 wrote. A conversion that adds volatile at an inner level needs const at every level above it. int** converts to volatile int* const*.
int** p = nullptr;
volatile int* const* q = p;
