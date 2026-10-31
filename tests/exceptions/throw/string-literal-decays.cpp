// tidal-rules: THR-2
// tidal-expect: run
int main() { try { throw "text"; } catch (const char* p) { return p[0] == 't' ? 0 : 1; } return 2; }
