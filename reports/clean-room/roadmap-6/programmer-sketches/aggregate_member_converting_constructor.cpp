struct Name {
  const char* s;
  Name(const char* p) : s(p) {}
};
struct Pair {
  Name first;
};
int main() {
  Pair p{"x"};
  return p.first.s[0] == 'x' ? 0 : 1;
}
