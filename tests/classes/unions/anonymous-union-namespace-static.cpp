// tidal-rules: UNI-7
// tidal-expect: run
static union { int i; float f; };
int main() { i = 2; return i == 2 ? 0 : 1; }
