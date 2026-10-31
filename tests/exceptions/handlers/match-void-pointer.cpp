// tidal-rules: TRY-2
// tidal-expect: run
int x;
int main() { try { throw &x; } catch (void* p) { return p == &x ? 0 : 1; } return 2; }
