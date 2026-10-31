// tidal-rules: TRY-1
// tidal-expect: run
int main() { try { throw 2.5; } catch (int) { return 1; } catch (double d) { return d == 2.5 ? 0 : 2; } return 3; }
