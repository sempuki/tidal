// tidal-rules: CPY-14 CVF-3 VIRT-3 VIRT-6 CAST-9 PACK-13
// tidal-expect: run
// tidal-note: Roadmap round 8, programmer sketch owner.cpp (reports/clean-room/roadmap-8/programmer.md, section 1.4). The rules are the ones the report names for this program and its reductions, and the rules of the main constructs it uses.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// An owning pointer and a growable array of owners: a move-only Box<T>
// with a converting move from Box<Derived>, explicit operator bool,
// [[nodiscard]] make_box with forwarding, a class hierarchy with
// override, final, and a covariant clone, dynamic_cast, and a vector
// in raw storage that moves its elements on growth only when the move
// cannot throw.
namespace std {
using size_t = decltype(sizeof 0);
}
inline void* operator new(std::size_t, void* p) noexcept { return p; }

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> constexpr typename remove_ref<T>::type&& move(T&& t) noexcept {
  return static_cast<typename remove_ref<T>::type&&>(t);
}
template <class T> constexpr T&& forward(typename remove_ref<T>::type& t) noexcept {
  return static_cast<T&&>(t);
}
template <class T, class U = T> constexpr T exchange(T& obj, U&& v) {
  T old = move(obj);
  obj = forward<U>(v);
  return old;
}
template <class From, class To>
concept convertible_to = requires(From* f) { static_cast<To*>(f); } &&
                         requires(void (*g)(To*), From* f) { g(f); };

template <class T>
class Box {
  T* p_ = nullptr;
  template <class U> friend class Box;
public:
  Box() = default;
  explicit Box(T* p) noexcept : p_(p) {}
  Box(const Box&) = delete;
  Box& operator=(const Box&) = delete;
  Box(Box&& o) noexcept : p_(exchange(o.p_, nullptr)) {}
  template <class U> requires convertible_to<U, T>
  Box(Box<U>&& o) noexcept : p_(exchange(o.p_, nullptr)) {}
  Box& operator=(Box&& o) noexcept {
    Box tmp(move(o));
    T* t = p_; p_ = tmp.p_; tmp.p_ = t;
    return *this;
  }
  ~Box() { delete p_; }
  explicit operator bool() const noexcept { return p_ != nullptr; }
  T& operator*() const { return *p_; }
  T* operator->() const noexcept { return p_; }
  T* get() const noexcept { return p_; }
};

template <class T, class... Args>
[[nodiscard]] Box<T> make_box(Args&&... args) {
  return Box<T>(new T(forward<Args>(args)...));
}

inline int alive = 0;

struct Shape {
  Shape() { ++alive; }
  Shape(const Shape&) { ++alive; }
  virtual ~Shape() { --alive; }
  virtual int area() const = 0;
  virtual Shape* clone() const = 0;
};

struct Rect : Shape {
  int w, h;
  Rect(int w_, int h_) : w(w_), h(h_) {}
  int area() const override { return w * h; }
  Rect* clone() const override { return new Rect(*this); }
};

struct Square final : Rect {
  explicit Square(int s) : Rect(s, s) {}
  Square* clone() const override { return new Square(*this); }
};

template <class T>
class Vec {
  T* data_ = nullptr;
  std::size_t size_ = 0, cap_ = 0;
  void grow() {
    std::size_t cap = cap_ ? cap_ * 2 : 1;
    T* fresh = static_cast<T*>(::operator new(cap * sizeof(T)));
    for (std::size_t i = 0; i < size_; ++i) {
      if constexpr (noexcept(T(move(data_[i])))) ::new (fresh + i) T(move(data_[i]));
      else ::new (fresh + i) T(data_[i]);
      data_[i].~T();
    }
    ::operator delete(data_);
    data_ = fresh;
    cap_ = cap;
  }
public:
  Vec() = default;
  Vec(const Vec&) = delete;
  ~Vec() {
    for (std::size_t i = size_; i > 0; --i) data_[i - 1].~T();
    ::operator delete(data_);
  }
  template <class... Args> T& emplace_back(Args&&... args) {
    if (size_ == cap_) grow();
    return *::new (data_ + size_++) T(forward<Args>(args)...);
  }
  T* begin() { return data_; }
  T* end() { return data_ + size_; }
  std::size_t size() const { return size_; }
};

static_assert(noexcept(Box<Shape>(move(*static_cast<Box<Rect>*>(nullptr)))));

int run() {
  Vec<Box<Shape>> shapes;
  shapes.emplace_back(make_box<Rect>(2, 3));
  shapes.emplace_back(make_box<Square>(4));
  Box<Rect> r = make_box<Rect>(1, 5);
  shapes.emplace_back(move(r));
  if (r) return 10;
  Box<Shape> copy(shapes.begin()[1]->clone());
  shapes.emplace_back(move(copy));

  int total = 0;
  int squares = 0;
  for (auto& b : shapes) {
    total += b->area();
    if (dynamic_cast<Square*>(b.get())) ++squares;
  }
  if (total != 6 + 16 + 5 + 16) return 1;
  if (squares != 2) return 2;
  if (alive != 4) return 3;
  return 0;
}

int main() {
  int rc = run();
  if (rc) return rc;
  return alive == 0 ? 0 : 4;
}
