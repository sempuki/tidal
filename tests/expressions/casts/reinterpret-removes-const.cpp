// tidal-rules: CAST-7
// tidal-expect: reject
const int c = 1;
long* p = reinterpret_cast<long*>(&c);
