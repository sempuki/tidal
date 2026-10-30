// tidal-rules: JMP-3
// tidal-expect: run
int f() { return 3.9; }
double g() { return 2; }
int main() { return f() == 3 && g() == 2.0 ? 0 : 1; }
