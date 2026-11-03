// tidal-rules: NAM-40
// tidal-expect: run
// tidal-note: Before the variable is declared, the name alone finds the class.
struct count { int v = 4; };
count early;
int count = 3;
int main() { return early.v + count == 7 ? 0 : 1; }
