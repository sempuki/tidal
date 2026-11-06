// Port of rust/shapes.rs: a trait object with a default method and
// dynamic dispatch.
//   trait Shape            -> abstract class with virtual functions (VIRT-1, VIRT-8)
//   default method name()  -> virtual function with a body in the base (VIRT-1)
//   Box<dyn Shape>         -> Owner<T>, the owner of copy-move.md (CPY-11, VIRT-7)
//   Vec<Box<dyn Shape>>    -> an array of Owner<Shape> (DCL-19)
//   iter().map().sum()     -> a function template over a range with a
//                             generic lambda (RFOR-1, LAM-1, LAM-P1)

using size_t = decltype(sizeof 0);

template <class T> class Owner {
  T* p;
  template <class U> friend class Owner;
public:
  constexpr explicit Owner(T* q) : p(q) {}
  Owner(const Owner&) = delete;
  Owner& operator=(const Owner&) = delete;
  constexpr Owner(Owner&& o) : p(o.p) { o.p = nullptr; }
  template <class U>
  constexpr Owner(Owner<U>&& o) : p(o.p) { o.p = nullptr; }
  constexpr ~Owner() { delete p; }
  constexpr T* operator->() const { return p; }
};

template <class T, class... Args>
constexpr Owner<T> make(Args&&... args) {
  return Owner<T>(new T(static_cast<Args&&>(args)...));
}

constexpr bool str_eq(const char* a, const char* b) {
  while (*a != '\0' && *a == *b) { ++a; ++b; }
  return *a == *b;
}

struct Shape {
  constexpr virtual ~Shape() = default;
  constexpr virtual double area() const = 0;
  constexpr virtual const char* name() const { return "shape"; }
};

struct Circle : Shape {
  double r;
  constexpr explicit Circle(double radius) : r(radius) {}
  constexpr double area() const override { return 3.0 * r * r; }
  constexpr const char* name() const override { return "circle"; }
};

struct Rect : Shape {
  double w, h;
  constexpr Rect(double width, double height) : w(width), h(height) {}
  constexpr double area() const override { return w * h; }
};

template <class Range, class F>
constexpr double sum_of(const Range& items, F f) {
  double total = 0.0;
  for (const auto& it : items) total += f(it);
  return total;
}

template <class Range, class P>
constexpr size_t count_if(const Range& items, P pred) {
  size_t n = 0;
  for (const auto& it : items)
    if (pred(it)) ++n;
  return n;
}

constexpr bool run() {
  Owner<Shape> shapes[] = {make<Circle>(1.0), make<Rect>(2.0, 3.0)};
  double total = sum_of(shapes, [](const auto& s) { return s->area(); });
  size_t named = count_if(shapes, [](const auto& s) { return !str_eq(s->name(), "shape"); });
  return total == 9.0 && named == 1;
}

static_assert(run());

int main() { return run() ? 0 : 1; }
