// tidal-rules: SINIT-4
// tidal-expect: run
int read_b();
int a = read_b();
inline int three() { return 3; }
int b = three();
int read_b() { return b; }
int main() { return a == 0 || a == 3 ? 0 : 1; }
