// tidal-rules: TRY-1
// tidal-expect: run
int outer() { try { try { throw 'c'; } catch (int) { return 1; } } catch (char c) { return c == 'c' ? 0 : 2; } return 3; }
int main() { return outer(); }
