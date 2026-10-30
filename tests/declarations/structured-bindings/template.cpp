// tidal-rules: SB-7
// tidal-expect: reject
int a[2];
template <class T> auto [x, y] = a;
