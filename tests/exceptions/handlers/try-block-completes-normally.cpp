// tidal-rules: TRY-1
// tidal-expect: run
int main() { int r = 1; try { r = 0; } catch (...) { r = 2; } return r; }
