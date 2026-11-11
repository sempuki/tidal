// A builder that returns *this with its own value category through this auto&&.
struct B { int w = 0, h = 0;
  template <class Self> Self&& width(this Self&& self, int v) { self.w = v; return static_cast<Self&&>(self); }
  template <class Self> Self&& height(this Self&& self, int v) { self.h = v; return static_cast<Self&&>(self); } };
int main() { B b = B{}.width(3).height(4); return b.w * b.h - 12; }
