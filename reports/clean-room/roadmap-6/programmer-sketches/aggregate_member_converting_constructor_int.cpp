struct Name {
  int n;
  Name(int p) : n(p) {}
};
struct Pair {
  Name first;
  int second;
};
int main() {
  Pair p{7, 1};
  return p.first.n == 7 && p.second == 1 ? 0 : 1;
}
