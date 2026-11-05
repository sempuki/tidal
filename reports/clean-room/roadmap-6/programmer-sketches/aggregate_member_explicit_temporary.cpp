struct Name {
  const char* s;
  Name(const char* p) : s(p) {}
};
struct Pair {
  Name first;
  int second;
};
int main() {
  Pair p{Name("x"), 1};
  return p.first.s[0] == 'x' && p.second == 1 ? 0 : 1;
}
