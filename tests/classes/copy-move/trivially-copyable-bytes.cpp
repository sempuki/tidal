// tidal-rules: CPY-19
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct P { int x; double y; };
int main() { P a{3, 2.5}, b{0, 0}; const unsigned char* src = reinterpret_cast<const unsigned char*>(&a); unsigned char* dst = reinterpret_cast<unsigned char*>(&b); for (unsigned i = 0; i < sizeof(P); ++i) dst[i] = src[i]; return b.x == 3 && b.y == 2.5 ? 0 : 1; }
