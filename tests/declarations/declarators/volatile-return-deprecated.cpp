// tidal-rules: CV-9
// tidal-expect: reject
// tidal-flags: -Werror -Wdeprecated
// tidal-note: Gap review 2026-10-30.
volatile int f();
void g(volatile int);
