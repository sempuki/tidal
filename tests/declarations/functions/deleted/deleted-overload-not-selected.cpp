// tidal-rules: DEL-1
// tidal-expect: run
int f(int) { return 0; }
int f(double) = delete;
int main() { return f(1); }
