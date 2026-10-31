// tidal-rules: TRY-3
// tidal-expect: run
int main() { try { throw 1; } catch (...) { return 0; } return 1; }
