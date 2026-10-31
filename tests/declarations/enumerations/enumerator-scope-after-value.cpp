// tidal-rules: ENUM-12
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
const int value = 12;
int get() { enum { value = value }; return value; }
int main() { return get() == 12 ? 0 : 1; }
