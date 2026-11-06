struct V {
  int n = 3;
  int get(this auto& self) { return self.n; }
};
int main() { V v; return v.get() - 3; }
