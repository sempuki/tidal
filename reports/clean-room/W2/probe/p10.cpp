struct Log {
  int seq[8]{};
  int n = 0;
  inline int mark(int id) { seq[n++] = id; return id; }
};
int main() { Log L; int s = L.mark(1) << L.mark(2); (void)s; return L.n; }
