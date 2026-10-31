// tidal-rules: PRIM-9
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
const char* get() { return "once"; }
int main() { const char* a = get(); const char* b = get(); return (a == b || a != b) ? 0 : 1; }
