// tidal-rules: CPY-23
// tidal-expect: reject
// tidal-error-clang: copy assignment operator is implicitly deleted
// tidal-error-gcc: use of deleted function .*operator=
// tidal-error-edg: operator=.* deleted function
int g;
struct S { int& r = g; };
void f(S& a, const S& b) { a = b; }
