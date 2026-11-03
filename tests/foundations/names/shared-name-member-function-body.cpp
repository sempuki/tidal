// tidal-rules: NAM-40
// tidal-expect: run
// tidal-note: In a member function body, the data member hides the nested class, also when the nested class comes after the data member.
struct shape {
  int size = 3;
  struct size { int v = 4; };
  int total() { struct size s; return size + s.v; }
};
int main() { shape sh; return sh.total() == 7 ? 0 : 1; }
