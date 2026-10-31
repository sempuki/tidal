// tidal-rules: MPTR-3
// tidal-expect: accept
struct I;
int h(I& i, int I::* pm) { return i.*pm; }
