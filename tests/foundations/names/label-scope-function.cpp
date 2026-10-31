// tidal-rules: NAM-7
// tidal-expect: run
int main() { int i = 0; goto later; i = 1; later: return i == 0 ? 0 : 1; }
