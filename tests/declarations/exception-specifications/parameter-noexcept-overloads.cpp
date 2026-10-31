// tidal-rules: EXS-2
// tidal-expect: accept
void take(void (*)());
void take(void (*)() noexcept);
void g() noexcept;
void h() { take(g); }
