// tidal-rules: SINIT-1
// tidal-expect: run
int read_k();
int early = read_k();
const int base = 4;
int k = base * 2;
int read_k() { return k; }
int main() { return early == 8 ? 0 : 1; }
