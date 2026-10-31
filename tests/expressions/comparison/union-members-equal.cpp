// tidal-rules: CMP-15
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
union U { int i; float f; };
int main() { U u{}; return static_cast<void*>(&u.i) == static_cast<void*>(&u.f) ? 0 : 1; }
