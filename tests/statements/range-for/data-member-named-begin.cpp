// tidal-rules: RFOR-2
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30. The compilers overrule the review: free begin and end are used.
struct R { int begin; int d[2]; };
int* begin(R& r) { return r.d; }
int* end(R& r) { return r.d + 2; }
void f(R& r) { for (int v : r) (void)v; }
