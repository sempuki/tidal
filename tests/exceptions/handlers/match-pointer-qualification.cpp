// tidal-rules: TRY-2
// tidal-expect: run
int x = 0;
int main() { try { throw &x; } catch (const int* p) { return p == &x ? 0 : 1; } return 2; }
