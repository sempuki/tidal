// tidal-rules: CAST-2
// tidal-expect: reject
int x;
long* p = static_cast<long*>(&x);
