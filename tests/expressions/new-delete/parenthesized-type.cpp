// tidal-rules: NEW-10
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int main() { int* p = new (int)(4); int (**q)() = new (int (*)()); bool ok = *p == 4 && q != nullptr; delete p; delete q; return ok ? 0 : 1; }
