// tidal-rules: CAST-7
// tidal-expect: reject
int x;
unsigned char n = reinterpret_cast<unsigned char>(&x);
