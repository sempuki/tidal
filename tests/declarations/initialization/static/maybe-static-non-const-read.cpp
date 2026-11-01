// tidal-rules: SINIT-4
// tidal-expect: run
int read_v();
int early = read_v();
int base = 4;
int v = base * 2;
int read_v() { return v; }
int main() { return (early == 0 || early == 8) && v == 8 ? 0 : 1; }
