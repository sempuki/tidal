// tidal-rules: SINIT-2
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
inline int first = 4;
inline int second = first + 1;
int main() { return second == 5 ? 0 : 1; }
