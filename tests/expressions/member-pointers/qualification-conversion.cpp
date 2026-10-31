// tidal-rules: MPTR-4
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct S { int m; int* p; };
int S::* pm = &S::m;
const int S::* pc = pm;
const volatile int S::* pcv = pm;
int* S::* pp = &S::p;
const int* const S::* ppc = pp;
