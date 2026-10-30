// tidal-rules: DTOR-6
// tidal-expect: run
int count = 0;
struct S { ~S() { ++count; } };
int main() {
  S* p = new S;
  p->~S();
  operator delete(p);
  return count == 1 ? 0 : 1;
}
