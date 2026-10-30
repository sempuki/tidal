// tidal-rules: SINIT-2
// tidal-expect: run
volatile int source = 7;
int read_y();
int early = read_y();
int y = source;
int read_y() { return y; }
int main() { return early == 0 && y == 7 ? 0 : 1; }
