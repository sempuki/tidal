// tidal-rules: SINIT-1
// tidal-expect: run
int read_k();
int early = read_k();
constexpr int square(int v) { return v * v; }
int k = square(4);
int read_k() { return k; }
int main() { return early == 16 ? 0 : 1; }
