// tidal-rules: SINIT-1
// tidal-expect: run
int target = 3;
int read_r();
int early = read_r();
int& ref = target;
int read_r() { return ref; }
int main() { return early == 3 ? 0 : 1; }
