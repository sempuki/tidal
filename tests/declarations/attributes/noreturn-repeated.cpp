// tidal-rules: ATTR-2
// tidal-expect: accept
// tidal-note: ATTR-P1. The first declaration has noreturn, a later declaration repeats the attribute, and a third declaration leaves it out.
[[noreturn]] void stop();
[[noreturn]] void stop();
void stop();
