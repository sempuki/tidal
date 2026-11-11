struct Builder {
  int w = 0, h = 0;
  template <class Self> constexpr Self&& width(this Self&& self, int v) { self.w = v; return static_cast<Self&&>(self); }
  template <class Self> constexpr Self&& height(this Self&& self, int v) { self.h = v; return static_cast<Self&&>(self); }
};
constexpr int area() { Builder b; b.width(3).height(5); return Builder{}.width(2).height(4).w * 10 + b.w * b.h; }
static_assert(area() == 35);
int main() { return area() - 35; }
