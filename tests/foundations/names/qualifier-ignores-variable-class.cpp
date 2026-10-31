// tidal-rules: NAM-15
// tidal-expect: accept
struct box { static constexpr int value = 1; };
int read() { int box = 0; return box::value + box; }
