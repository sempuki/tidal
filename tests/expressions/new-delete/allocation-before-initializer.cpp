// tidal-rules: NEW-12
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }

struct S {
  int v;
  S(int x) : v(x) {}
  static void* operator new(decltype(sizeof 0)) { mark(1); static unsigned char buf[sizeof(int) * 4]; return buf; }
  static void operator delete(void*) {}
};
int arg() { mark(2); return 3; }
int main() { S* p = new S(arg()); return p->v == 3 && trace == 12 ? 0 : 1; }
