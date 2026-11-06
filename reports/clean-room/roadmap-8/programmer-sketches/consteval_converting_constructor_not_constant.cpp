void fail();

struct checked {
  int n;
  consteval checked(int v) : n(v) { if (v < 0) fail(); }
};

int take(checked c) { return c.n; }

int main() { return take(-1); }
