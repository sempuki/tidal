// tidal-rules: NEW-10
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int seen = 0;
unsigned char pool[64];
void* operator new(decltype(sizeof 0) n, int tag) { seen = tag; return pool; }
int main() { int* p = new (7) int(3); return seen == 7 && *p == 3 ? 0 : 1; }
