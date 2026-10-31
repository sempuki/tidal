// tidal-rules: TRY-1
// tidal-expect: run
int main() { try { throw 3; } catch (int v) { return v == 3 ? 0 : 1; } return 2; }
