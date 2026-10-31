// tidal-rules: REF-14 REF-4
// tidal-expect: run
struct Point { int x, y; };

int destroyed = 0;
struct Box {
  int w, h;
  Box(int a, int b) : w(a), h(b) {}
  ~Box() { ++destroyed; }
};

int main() {
  const Point& p{1, 2};
  Point&& q{3};
  if (p.x != 1 || p.y != 2 || q.x != 3 || q.y != 0) return 1;
  {
    const Box& b{4, 5};
    if (destroyed != 0) return 2;
    if (b.w * b.h != 20) return 3;
  }
  if (destroyed != 1) return 4;
  return 0;
}
