// tidal-rules: CAST-7
// tidal-expect: reject
// tidal-error-clang: loses information
// tidal-error-gcc: loses precision
// tidal-error-edg: invalid type conversion
int x;
unsigned char n = reinterpret_cast<unsigned char>(&x);
