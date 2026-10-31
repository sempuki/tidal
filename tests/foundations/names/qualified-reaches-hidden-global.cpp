// tidal-rules: NAM-2
// tidal-expect: run
int x = 1;
int main() { int x = 2; return ::x == 1 && x == 2 ? 0 : 1; }
