// tidal-rules: TRY-2
// tidal-expect: run
int f() { return 7; }
int main() { try { throw f; } catch (int (*p)()) { return p() == 7 ? 0 : 1; } return 2; }
