// tidal-rules: TRY-9
// tidal-expect: run
int main() try { throw 1; } catch (int) { return 0; }
