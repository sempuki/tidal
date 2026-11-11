// tidal-rules: ACC-5 ATTR-9 PACK-13 OPR-6 CPY-2 DTOR-6 VIRT-1
// tidal-expect: run
// tidal-note: Roadmap round 10, programmer sketch smart_ptr_first_try.cpp (reports/clean-room/roadmap-10/programmer.md, section 1.4). A first try that builds clean on GCC and Clang is a program test beside the last version (decision 0061). The rules are the ones the report names for this program and its reductions, and the rules of the main constructs it uses.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Owning pointers: a Unique<T, D> with an empty deleter stored as a
// [[no_unique_address]] member, a converting move from Unique<Derived>,
// a Shared<T> with a control block that holds the object in place, a Weak<T>
// that locks, an aliasing constructor, make_shared that forwards a pack into
// the control block's constructor, and owners in a container of owners.
namespace std { using size_t = decltype(sizeof 0); }

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };
template <class T> constexpr typename remove_ref<T>::type&& move(T&& t) noexcept {
  return static_cast<typename remove_ref<T>::type&&>(t);
}
template <class T> constexpr T&& forward(typename remove_ref<T>::type& t) noexcept {
  return static_cast<T&&>(t);
}
template <class From, class To>
concept convertible_to = requires(From* f) { static_cast<To*>(f); } &&
                         requires(void (*g)(To*), From* f) { g(f); };

inline int destroyed = 0;

template <class T> struct DefaultDelete {
  constexpr DefaultDelete() = default;
  template <class U> requires convertible_to<U, T>
  constexpr DefaultDelete(DefaultDelete<U>) noexcept {}
  void operator()(T* p) const noexcept { delete p; }
};

template <class T, class D = DefaultDelete<T>>
class Unique {
  T* p_ = nullptr;
  [[no_unique_address]] D d_{};
  template <class, class> friend class Unique;
public:
  constexpr Unique() = default;
  explicit Unique(T* p) noexcept : p_(p) {}
  Unique(T* p, D d) noexcept : p_(p), d_(move(d)) {}
  Unique(const Unique&) = delete;
  Unique& operator=(const Unique&) = delete;
  Unique(Unique&& o) noexcept : p_(o.p_), d_(move(o.d_)) { o.p_ = nullptr; }
  template <class U, class E> requires convertible_to<U, T>
  Unique(Unique<U, E>&& o) noexcept : p_(o.p_), d_(move(o.d_)) { o.p_ = nullptr; }
  Unique& operator=(Unique&& o) noexcept {
    if (this != &o) { reset(o.p_); o.p_ = nullptr; d_ = move(o.d_); }
    return *this;
  }
  ~Unique() { reset(); }
  void reset(T* q = nullptr) noexcept { T* old = p_; p_ = q; if (old) d_(old); }
  T* release() noexcept { T* q = p_; p_ = nullptr; return q; }
  T* get() const noexcept { return p_; }
  T& operator*() const { return *p_; }
  T* operator->() const noexcept { return p_; }
  explicit operator bool() const noexcept { return p_ != nullptr; }
};

template <class T, class... A> Unique<T> make_unique(A&&... a) {
  return Unique<T>(new T(forward<A>(a)...));
}

struct ControlBase {
  long strong = 1, weak = 0;
  virtual void destroy_object() noexcept = 0;
  virtual ~ControlBase() = default;
  void add_strong() noexcept { ++strong; }
  void drop_strong() noexcept {
    if (--strong == 0) { destroy_object(); if (weak == 0) delete this; }
  }
  void drop_weak() noexcept { if (--weak == 0 && strong == 0) delete this; }
};

template <class T> struct InPlace final : ControlBase {
  union { T value; };
  template <class... A> explicit InPlace(A&&... a) : value(forward<A>(a)...) {}
  void destroy_object() noexcept override { value.~T(); }
  ~InPlace() override {}
};

template <class T> class Weak;

template <class T> class Shared {
  T* p_ = nullptr;
  ControlBase* c_ = nullptr;
  template <class> friend class Shared;
  template <class> friend class Weak;
  template <class U, class... A> friend Shared<U> make_shared(A&&...);
  Shared(T* p, ControlBase* c) noexcept : p_(p), c_(c) {}
public:
  Shared() = default;
  Shared(const Shared& o) noexcept : p_(o.p_), c_(o.c_) { if (c_) c_->add_strong(); }
  Shared(Shared&& o) noexcept : p_(o.p_), c_(o.c_) { o.p_ = nullptr; o.c_ = nullptr; }
  template <class U> requires convertible_to<U, T>
  Shared(const Shared<U>& o) noexcept : p_(o.p_), c_(o.c_) { if (c_) c_->add_strong(); }
  template <class U>
  Shared(const Shared<U>& owner, T* alias) noexcept : p_(alias), c_(owner.c_) {
    if (c_) c_->add_strong();
  }
  Shared& operator=(Shared o) noexcept {
    T* tp = p_; p_ = o.p_; o.p_ = tp;
    ControlBase* tc = c_; c_ = o.c_; o.c_ = tc;
    return *this;
  }
  ~Shared() { if (c_) c_->drop_strong(); }
  T* get() const noexcept { return p_; }
  T& operator*() const noexcept { return *p_; }
  T* operator->() const noexcept { return p_; }
  long use_count() const noexcept { return c_ ? c_->strong : 0; }
  explicit operator bool() const noexcept { return p_ != nullptr; }
};

template <class T, class... A> Shared<T> make_shared(A&&... a) {
  auto* c = new InPlace<T>(forward<A>(a)...);
  return Shared<T>(&c->value, c);
}

template <class T> class Weak {
  T* p_ = nullptr;
  ControlBase* c_ = nullptr;
public:
  Weak(const Shared<T>& s) noexcept : p_(s.p_), c_(s.c_) { if (c_) ++c_->weak; }
  Weak(const Weak&) = delete;
  ~Weak() { if (c_) c_->drop_weak(); }
  bool expired() const noexcept { return !c_ || c_->strong == 0; }
  Shared<T> lock() const noexcept {
    if (expired()) return Shared<T>();
    c_->add_strong();
    return Shared<T>(p_, c_);
  }
};

struct Shape {
  virtual ~Shape() { ++destroyed; }
  virtual int area() const = 0;
};
struct Rect : Shape {
  int w, h;
  Rect(int w, int h) : w(w), h(h) {}
  int area() const override { return w * h; }
};
struct Point { int x, y; };
struct Pair { Point a, b; Pair(Point a, Point b) : a(a), b(b) {} };

struct CountingDelete {
  int* count;
  void operator()(int* p) const noexcept { ++*count; delete p; }
};

int main() {
  int fails = 0;
  {
    Unique<Rect> r = make_unique<Rect>(3, 4);
    Unique<Shape> s = move(r);
    if (r || !s || s->area() != 12) ++fails;
    s.reset();
    if (destroyed != 1) ++fails;
    static_assert(sizeof(Unique<int>) == sizeof(int*));
  }
  {
    int count = 0;
    {
      Unique<int, CountingDelete> u(new int(7), CountingDelete{&count});
      Unique<int, CountingDelete> v = move(u);
      if (*v != 7 || u.get() != nullptr) ++fails;
    }
    if (count != 1) ++fails;
  }
  {
    Shared<Pair> p = make_shared<Pair>(Point{1, 2}, Point{3, 4});
    Shared<Point> second(p, &p->b);
    if (p.use_count() != 2 || second->x != 3) ++fails;
    Weak<Pair> w(p);
    p = Shared<Pair>();
    if (w.expired()) ++fails;
    second = Shared<Point>();
    if (!w.expired() || w.lock()) ++fails;
  }
  {
    Shared<Rect> r = make_shared<Rect>(2, 5);
    Shared<Shape> s = r;
    Shared<Shape> t;
    t = s;
    if (s.use_count() != 3 || t->area() != 10) ++fails;
  }
  if (destroyed != 2) ++fails;
  {
    Unique<Unique<int>> nested = make_unique<Unique<int>>(new int(5));
    if (**nested != 5) ++fails;
  }
  return fails;
}
