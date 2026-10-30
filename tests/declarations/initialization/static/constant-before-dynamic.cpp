// tidal-rules: SINIT-1
// tidal-expect: run
int read_later();
int early = read_later();
int later = 5;
int read_later() { return later; }
int main() { return early == 5 ? 0 : 1; }
