template <class T> struct Box {
  T* p;
  explicit Box(T* q) : p(q) {}
  Box(Box&& o) noexcept : p(o.p) { o.p = nullptr; }
  ~Box() { delete p; }
  template <class U> Box(Box<U>&& o) noexcept : p(o.p) { o.p = nullptr; }
};
struct B { virtual ~B() = default; };
struct D : B {};
Box<D> makeD() { return Box<D>(new D); }
Box<B> f() { return makeD(); }
int main() { Box<B> b = f(); return b.p ? 0 : 1; }
