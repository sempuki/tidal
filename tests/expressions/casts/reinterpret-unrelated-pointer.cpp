// tidal-rules: CAST-6
// tidal-expect: accept
int x;
long* p = reinterpret_cast<long*>(&x);
char* c = reinterpret_cast<char*>(&x);
