// tidal-rules: INIT-4
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int main() { bool a(nullptr); bool b{nullptr}; return !a && !b ? 0 : 1; }
