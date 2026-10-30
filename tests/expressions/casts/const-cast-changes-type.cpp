// tidal-rules: CAST-4
// tidal-expect: reject
const int c = 1;
long* p = const_cast<long*>(&c);
