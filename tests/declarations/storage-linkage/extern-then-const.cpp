// tidal-rules: LNK-3
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
// tidal-sources: extern-then-const.tu.cpp
extern const int shared_k;
const int shared_k = 5;
int read_k();
int main() { return read_k() == 5 ? 0 : 1; }
