// tidal-rules: UNW-7
// tidal-expect: run-fail
volatile int destroyed = 0;
struct D { ~D() { destroyed = 1; } };
void f() { D d; throw 0; }
int main() {
  f();
  return 0;
}
