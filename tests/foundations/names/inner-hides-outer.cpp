// tidal-rules: NAM-2
// tidal-expect: run
int x = 1;
int main() { int x = 2; { int x = 3; if (x != 3) return 1; } return x == 2 ? 0 : 1; }
