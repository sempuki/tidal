// tidal-rules: PFX-10
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int calls = 0;
struct S { static int count; };
int S::count = 5;
S& get() { static S s; ++calls; return s; }
int main() { int v = get().count; return v == 5 && calls == 1 ? 0 : 1; }
