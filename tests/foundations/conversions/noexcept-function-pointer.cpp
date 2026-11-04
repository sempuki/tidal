// tidal-rules: CONV-10
// tidal-expect: accept
void f() noexcept;
void (*p)() = f;
void (*q)() noexcept = f;
