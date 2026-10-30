// tidal-rules: CAST-4
// tidal-expect: reject
int f(const int v) { return const_cast<int>(v); }
