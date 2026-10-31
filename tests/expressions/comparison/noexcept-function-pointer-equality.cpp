// tidal-rules: CMP-19
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31 (third).
void g() noexcept;
void (*p)() = nullptr;
bool f() { return p == &g; }
