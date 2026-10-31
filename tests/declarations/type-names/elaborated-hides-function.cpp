// tidal-rules: TYN-4
// tidal-expect: run
struct stat { int size = 8; };
int stat(int x) { return x; }
int main() { struct stat s; return s.size == 8 && stat(1) == 1 ? 0 : 1; }
