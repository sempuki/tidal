// tidal-rules: EXS-2
// tidal-expect: reject
void f() noexcept;
void f() noexcept(false) {}
