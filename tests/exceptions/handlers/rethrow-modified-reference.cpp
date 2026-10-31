// tidal-rules: TRY-8
// tidal-expect: run
int main() { try { try { throw 1; } catch (int& v) { v = 2; throw; } } catch (int v) { return v == 2 ? 0 : 1; } return 3; }
