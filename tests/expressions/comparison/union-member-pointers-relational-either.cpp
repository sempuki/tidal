// tidal-rules: CMP-8
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
union U { int i; int j; };
int main() { U u{}; bool r = &u.i < &u.j; (void)r; return 0; }
