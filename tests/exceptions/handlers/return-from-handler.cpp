// tidal-rules: TRY-15
// tidal-expect: run
int f() { try { throw 1; } catch (int) { return 5; } return 2; }
int main() { return f() == 5 ? 0 : 1; }
