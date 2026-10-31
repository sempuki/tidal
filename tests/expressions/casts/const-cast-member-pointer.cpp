// tidal-rules: CAST-4
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct S { int m; };
const int S::* pc = &S::m;
int S::* pm = const_cast<int S::*>(pc);
const int* const* pp = nullptr;
int** q = const_cast<int**>(pp);
