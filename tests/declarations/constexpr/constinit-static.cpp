// tidal-rules: CINIT-1
// tidal-expect: run
constinit int counter = 5;
int main() { counter += 1; return counter == 6 ? 0 : 1; }
