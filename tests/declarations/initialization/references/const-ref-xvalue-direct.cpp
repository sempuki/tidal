// tidal-rules: REF-2
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int g = 1;
int main() { const int& r = static_cast<int&&>(g); g = 5; return r == 5 ? 0 : 1; }
