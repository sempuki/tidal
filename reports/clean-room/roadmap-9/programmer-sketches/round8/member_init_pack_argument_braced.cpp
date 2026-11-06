struct point {
  int x, y;
  point(int a, int b) : x(a), y(b) {}
};

template <class... A>
struct wrapper {
  point p;
  wrapper(A... a) : p{a...} {}
};

int main() {
  wrapper<int, int> w(2, 3);
  return w.p.x + w.p.y == 5 ? 0 : 1;
}
