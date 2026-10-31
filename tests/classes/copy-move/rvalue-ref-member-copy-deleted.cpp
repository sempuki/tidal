// tidal-rules: CPY-23
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted|call to deleted
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
// tidal-note: Gap review 2026-10-31.
int g;
struct S { int&& r; };
S make();
void f(S& a) { S b(a); }
