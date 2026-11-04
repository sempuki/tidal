// tidal-rules: VC-3
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
struct S { static int count; };
int S::count = 0;
S make();
int* p = &make().count;
