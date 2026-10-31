// tidal-rules: NEW-19
// tidal-expect: run
int tagged = 0;
void* operator new(decltype(sizeof 0) n, int tag) { tagged += tag; return ::operator new(n); }
struct Pool {
  static void* operator new(decltype(sizeof 0) n);
  static void operator delete(void* p);
  int v;
};
int pooled = 0;
void* Pool::operator new(decltype(sizeof 0) n) { ++pooled; return ::operator new(n); }
void Pool::operator delete(void* p) { ::operator delete(p); }
int main() {
  int* t = new (5) int(1);
  Pool* p = new Pool{3};
  bool ok = tagged == 5 && *t == 1 && pooled == 1 && p->v == 3;
  delete t;
  delete p;
  return ok ? 0 : 1;
}
