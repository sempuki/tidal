// tidal-rules: REF-4
// tidal-expect: run
// tidal-note: Gap review 2026-10-30.
struct Note { int v; };
const Note& global = Note{7};
int main() { return global.v == 7 ? 0 : 1; }
