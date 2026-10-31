// tidal-rules: CPY-23
// tidal-expect: reject
// tidal-error-clang: copy assignment operator is implicitly deleted
// tidal-error-gcc: use of deleted function .*operator=
// tidal-error-edg: operator=.* deleted function
struct S { const int c = 1; };
void f(S& a, S& b) { a = static_cast<S&&>(b); }
