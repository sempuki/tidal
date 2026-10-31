// tidal-rules: TRY-2
// tidal-expect: run
int main() { try { throw 5; } catch (int& v) { v = 6; return v == 6 ? 0 : 1; } return 2; }
