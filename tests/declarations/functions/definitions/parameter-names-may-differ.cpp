// tidal-rules: FN-16
// tidal-expect: run
int scale(int factor, int value);
int scale(int, int);
int scale(int value, int factor);
int scale(int left, int right) { return left * 10 + right; }

struct Shape {
  virtual int area(int width, int height) const { return width * height; }
  virtual ~Shape() = default;
};
struct Half : Shape {
  int area(int w, int h) const override { return w * h / 2; }
};

int main() {
  if (scale(1, 2) != 12) return 1;
  Half half;
  const Shape& shape = half;
  if (shape.area(4, 3) != 6) return 2;
  return 0;
}
