// tidal-rules: TRY-8
// tidal-expect: run
int main() { try { try { throw 3; } catch (int) { throw; } } catch (int v) { return v == 3 ? 0 : 1; } return 2; }
