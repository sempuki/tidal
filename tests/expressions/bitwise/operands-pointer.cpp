// tidal-rules: BIT-3
// tidal-expect: reject
int x;
int* p = &x;
auto r = p | 1;
