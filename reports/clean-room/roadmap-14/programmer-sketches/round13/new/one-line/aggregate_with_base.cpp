// An aggregate with a base class, initialized with a nested list.
struct Point { int x, y; };
struct Pixel : Point { int color; };
int main() {
  Pixel p{{1, 2}, 7};
  Pixel q{3, 4, 5};
  return (p.x + p.y + p.color == 10 && q.y == 4 && q.color == 5) ? 0 : 1;
}
