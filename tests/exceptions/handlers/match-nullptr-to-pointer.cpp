// tidal-rules: TRY-2
// tidal-expect: run
int main() { try { throw nullptr; } catch (int* p) { return p == nullptr ? 0 : 1; } return 2; }
