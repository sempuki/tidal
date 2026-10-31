// tidal-rules: TRY-8
// tidal-expect: run
int main() { try { try { throw 1; } catch (int) { throw 2.0; } } catch (double d) { return d == 2.0 ? 0 : 1; } return 3; }
