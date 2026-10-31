// tidal-rules: MPTR-1
// tidal-expect: accept
struct Later;
int Later::* pm = nullptr;
void (Later::* pf)() = nullptr;
