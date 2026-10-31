// tidal-rules: DCMP-6
// tidal-expect: run
int used = 0;
struct S { int v;
  bool operator==(const S& o) const { return v == o.v; }
  bool operator!=(const S& o) const { ++used; return v != o.v; } };
int main() { S a{1}, b{2}; bool r = a != b; return r && used == 1 ? 0 : 1; }
