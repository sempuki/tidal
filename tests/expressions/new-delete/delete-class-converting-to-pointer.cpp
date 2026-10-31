// tidal-rules: NEW-15
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct Owner { int* p; operator int*() const { return p; } };
int main() { Owner o{new int(1)}; delete o; return 0; }
