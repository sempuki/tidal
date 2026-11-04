// tidal-rules: VC-3
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
void f();
void (*p)() = &static_cast<void (&&)()>(f);
