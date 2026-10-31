// tidal-rules: CX-1
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
int main() { constexpr const int& r = 1; return r; }
