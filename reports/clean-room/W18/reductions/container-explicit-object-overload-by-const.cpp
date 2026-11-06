struct V {
  int n = 0;
  int f(this V& self) { return self.n; }
  int f(this const V& self) { return self.n + 1; }
};
int main() { V v; const V& c = v; return v.f() + c.f() - 1; }
