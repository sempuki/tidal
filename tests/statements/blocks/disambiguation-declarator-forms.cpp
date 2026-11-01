// tidal-rules: STMT-6
// tidal-expect: run
struct Name { int v = 0; Name() {} Name(int x) : v(x) {} };
int main() {
  int a = 3;
  Name(b);
  Name(c)(a);
  Name(*p) = nullptr;
  int(d)[2] = {1, 2};
  return b.v == 0 && c.v == 3 && p == nullptr && d[1] == 2 ? 0 : 1;
}
