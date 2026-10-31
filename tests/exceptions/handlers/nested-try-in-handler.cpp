// tidal-rules: TRY-8
// tidal-expect: run
int main() { try { throw 1; } catch (int) { try { throw 2; } catch (int v) { return v == 2 ? 0 : 1; } } return 3; }
