// tidal-rules: LIST-4
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct Empty {};
struct S : Empty { int v; };
int main() { S s{{}, 1}; return s.v == 1 ? 0 : 1; }
