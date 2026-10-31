// tidal-rules: TRY-2
// tidal-expect: run
int main() { try { throw 4; } catch (const int& v) { return v == 4 ? 0 : 1; } return 2; }
