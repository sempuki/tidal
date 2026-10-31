// tidal-rules: MPTR-11
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct S { int* p; };
int* S::* pp = &S::p;
const int* S::* ppc = pp;
