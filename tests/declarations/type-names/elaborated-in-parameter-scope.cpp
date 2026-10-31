// tidal-rules: TYN-4
// tidal-expect: run
void take(struct Tag* p);
struct Tag { int v = 3; };
void take(Tag* p) { p->v = 4; }
int main() { Tag t; take(&t); return t.v == 4 ? 0 : 1; }
