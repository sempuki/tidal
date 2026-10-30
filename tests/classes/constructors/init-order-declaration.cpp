// tidal-rules: CTOR-6
// tidal-expect: run
int trace = 0;
int mark(int d) { trace = trace * 10 + d; return d; }
struct B1 { int v = mark(1); };
struct B2 { int v = mark(2); };
struct S : B1, B2 {
  int a;
  int b;
  S() : b(mark(4)), a(mark(3)) { mark(5); }
};
int main() { S s; return trace == 12345 ? 0 : 1; }
