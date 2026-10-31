// tidal-rules: UNEV-1
// tidal-expect: run
int declared_only();
struct Big { char data[64]; };
Big make_big();
int main() { decltype(declared_only()) x = 1; return sizeof(make_big()) == 64 && x == 1 ? 0 : 1; }
