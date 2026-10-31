// tidal-rules: THR-6
// tidal-expect: run
int main() { try { try { throw 5; } catch (int& e) { e = 6; throw; } } catch (int e) { return e == 6 ? 0 : 1; } return 2; }
