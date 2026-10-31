// tidal-rules: CVF-3
// tidal-expect: run
struct Handle { int fd; explicit operator bool() const { return fd >= 0; } };
int main() { Handle h{3}; Handle bad{-1};
  if (!h) return 1;
  if (bad) return 1;
  bool both = h && !bad;
  int pick = h ? 1 : 2;
  while (bad) return 1;
  return both && pick == 1 ? 0 : 1; }
