struct Log {
  int seq[8]{};
  int n = 0;
  constexpr int mark(int id) { seq[n++] = id; return id; }
};
constexpr int shift_calls() { Log L; int s = L.mark(1) << L.mark(2); (void)s; return L.n; }
constexpr int shift_calls_r() { Log L; int s = L.mark(1) >> L.mark(2); (void)s; return L.n; }
int calls = 0;
int mark(int v) { ++calls; return v; }
int main() {
  constexpr int c = shift_calls();
  constexpr int cr = shift_calls_r();
  int s = mark(1) << mark(2);
  int rt = calls;
  volatile int keep = s;
  (void)keep;
  return c * 100 + cr * 10 + rt;
}
