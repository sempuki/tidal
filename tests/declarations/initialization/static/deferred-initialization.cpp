// tidal-rules: SINIT-5
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
int initialized = 0;
struct Mark { Mark() { initialized = 1; } };
Mark mark;
int main() { return initialized == 0 || initialized == 1 ? 0 : 1; }
