template <class D> struct Shape { int area() const { return static_cast<const D&>(*this).area_impl(); } };
struct Sq : Shape<Sq> { int s; int area_impl() const { return s * s; } };
template <class D> int twice(const Shape<D>& x) { return 2 * x.area(); }
int main() { Sq q; q.s = 3; return twice(q) == 18 ? 0 : 1; }
