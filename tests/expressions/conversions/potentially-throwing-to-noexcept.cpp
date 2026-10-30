// tidal-rules: CONV-10
// tidal-expect: reject
void g();
void (*q)() noexcept = g;
