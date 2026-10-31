// tidal-rules: UNEV-1
// tidal-expect: run
extern int never_defined;
int main() { return sizeof(never_defined) == sizeof(int) ? 0 : 1; }
