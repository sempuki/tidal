// tidal-rules: TRY-8
// tidal-expect: run
void again() { throw; }
int main() { try { try { throw 4; } catch (int) { again(); } } catch (int v) { return v == 4 ? 0 : 1; } return 2; }
