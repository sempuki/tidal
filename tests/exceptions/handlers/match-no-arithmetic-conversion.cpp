// tidal-rules: TRY-2
// tidal-expect: run
int main() { try { throw 'c'; } catch (int) { return 1; } catch (char) { return 0; } return 2; }
