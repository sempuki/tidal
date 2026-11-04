// tidal-rules: OBJ-4
// tidal-expect: run
// tidal-note: s is constructed first and t at its first use. t ends with the main thread, before the static object s. The destructor of s checks the order. The write through a null pointer runs only when the order is wrong.
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct S {
  int id;
  S(int i) : id(i) { mark(i); }
  ~S() { mark(id); if (id == 1 && trace != 1221) *(volatile int*)0 = 0; }
};
S s(1);
thread_local S t(2);
int main() { return t.id == 2 ? 0 : 1; }
