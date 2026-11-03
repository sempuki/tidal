// tidal-rules: DCMP-4
// tidal-expect: run
int calls = 0;
struct S {
  int x;
  bool operator==(const S& o) const { ++calls; return x == o.x; }
  bool operator!=(const S&) const = default;
};
int main() {
  bool differ = S{1} != S{2};
  bool same = S{1} != S{1};
  return differ && !same && calls == 2 ? 0 : 1;
}
