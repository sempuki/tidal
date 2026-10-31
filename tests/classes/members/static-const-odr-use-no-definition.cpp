// tidal-rules: CLS-21
// tidal-expect: link-fail
// tidal-note: Gap review 2026-10-31.
struct S { static const int k = 3; };
const int& r = S::k;
int main() { return r; }
