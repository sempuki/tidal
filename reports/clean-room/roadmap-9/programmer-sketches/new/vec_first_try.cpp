// A generic growable array in raw storage: emplace_back that forwards a
// pack into a placement new, push_back for lvalues and rvalues, growth that
// moves only when the move cannot throw, copy and move of the whole array,
// a variadic factory that expands a pack in a fold, erase_if with a lambda,
// a nested Vec<Vec<int>>, and element types that convert from other
// classes and from string literals through converting constructors.
namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* p) noexcept { return p; }

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };
template <class T> constexpr typename remove_ref<T>::type&& move(T&& t) noexcept {
  return static_cast<typename remove_ref<T>::type&&>(t);
}
template <class T> constexpr T&& forward(typename remove_ref<T>::type& t) noexcept {
  return static_cast<T&&>(t);
}
template <class T> T&& declval() noexcept;
template <class T> constexpr bool nothrow_movable = noexcept(T(declval<T&&>()));

inline int alive = 0;    // objects of Tracked alive now
inline int copies = 0;   // copies of Tracked made

template <class T>
class Vec {
  T* data_ = nullptr;
  std::size_t size_ = 0;
  std::size_t cap_ = 0;

  void grow(std::size_t want) {
    if (want <= cap_) return;
    std::size_t ncap = cap_ ? cap_ * 2 : 2;
    while (ncap < want) ncap *= 2;
    T* nd = static_cast<T*>(::operator new(ncap * sizeof(T)));
    for (std::size_t i = 0; i < size_; ++i) {
      if constexpr (nothrow_movable<T>) ::new (static_cast<void*>(nd + i)) T(move(data_[i]));
      else ::new (static_cast<void*>(nd + i)) T(data_[i]);
      data_[i].~T();
    }
    ::operator delete(data_);
    data_ = nd;
    cap_ = ncap;
  }

public:
  Vec() = default;
  Vec(const Vec& o) {
    grow(o.size_);
    for (const T& x : o) emplace_back(x);
  }
  Vec(Vec&& o) noexcept : data_(o.data_), size_(o.size_), cap_(o.cap_) {
    o.data_ = nullptr;
    o.size_ = o.cap_ = 0;
  }
  Vec& operator=(Vec o) noexcept {   // copy and swap
    T* d = data_; data_ = o.data_; o.data_ = d;
    std::size_t s = size_; size_ = o.size_; o.size_ = s;
    std::size_t c = cap_; cap_ = o.cap_; o.cap_ = c;
    return *this;
  }
  ~Vec() { clear(); ::operator delete(data_); }

  template <class... Args>
  T& emplace_back(Args&&... args) {
    grow(size_ + 1);
    T* p = ::new (static_cast<void*>(data_ + size_)) T(forward<Args>(args)...);
    ++size_;
    return *p;
  }
  void push_back(const T& x) { emplace_back(x); }
  void push_back(T&& x) { emplace_back(move(x)); }
  void pop_back() { data_[--size_].~T(); }
  void clear() { while (size_) pop_back(); }

  template <class Pred>
  std::size_t erase_if(Pred pred) {
    std::size_t out = 0;
    for (std::size_t i = 0; i < size_; ++i)
      if (!pred(data_[i])) {
        if (out != i) data_[out] = move(data_[i]);
        ++out;
      }
    std::size_t removed = size_ - out;
    while (size_ > out) pop_back();
    return removed;
  }

  std::size_t size() const { return size_; }
  bool empty() const { return size_ == 0; }
  T& operator[](std::size_t i) { return data_[i]; }
  const T& operator[](std::size_t i) const { return data_[i]; }
  T* begin() { return data_; }
  T* end() { return data_ + size_; }
  const T* begin() const { return data_; }
  const T* end() const { return data_ + size_; }
};

template <class T, class... U>
Vec<T> vec_of(U&&... us) {
  Vec<T> v;
  (v.emplace_back(forward<U>(us)), ...);
  return v;
}

// A name that converts from a string literal and from a Token (another class).
struct Token {
  const char* text;
};
class Name {
  char buf_[16] = {};
  int len_ = 0;
public:
  Name() = default;
  Name(const char* s) {
    while (s[len_] && len_ < 15) { buf_[len_] = s[len_]; ++len_; }
  }
  Name(Token t) : Name(t.text) {}
  int size() const { return len_; }
  friend bool operator==(const Name& a, const Name& b) {
    if (a.len_ != b.len_) return false;
    for (int i = 0; i < a.len_; ++i)
      if (a.buf_[i] != b.buf_[i]) return false;
    return true;
  }
};

struct Tracked {
  Name name;
  int age;
  Tracked(Name n, int a) : name(n), age(a) { ++alive; }
  Tracked(const Tracked& o) : name(o.name), age(o.age) { ++alive; ++copies; }
  Tracked(Tracked&& o) noexcept : name(o.name), age(o.age) { ++alive; }
  Tracked& operator=(const Tracked&) = default;
  Tracked& operator=(Tracked&&) = default;
  ~Tracked() { --alive; }
};

int check_tracked() {
  {
    Vec<Tracked> people;
    people.emplace_back("ann", 31);           // const char* to Name, in the pack
    people.emplace_back(Token{"bob"}, 42);    // Token to Name
    Tracked carl{"carl", 25};
    people.push_back(carl);                   // one copy
    people.push_back(Tracked{"dora", 19});    // a move
    for (int i = 0; i < 5; ++i) people.emplace_back("x", i);
    if (people.size() != 9) return 1;
    if (copies != 1) return 2;                // growth moved, it did not copy
    if (!(people[1].name == Token{"bob"})) return 3;   // Token binds to const Name&
    if (!(people[0].name == "ann")) return 4;
    std::size_t gone = people.erase_if([](const Tracked& t) { return t.name == "x"; });
    if (gone != 5 || people.size() != 4) return 5;
    int total = 0;
    for (const auto& [name, age] : people) total += age + name.size();
    if (total != 31 + 42 + 25 + 19 + 3 + 3 + 4 + 4) return 6;
    Vec<Tracked> copy = people;
    copy.pop_back();
    if (copy.size() != 3 || people.size() != 4) return 7;
    Vec<Tracked> moved = move(copy);
    if (moved.size() != 3 || !copy.empty()) return 8;
  }
  return alive == 0 ? 0 : 9;
}

int check_nested() {
  Vec<Vec<int>> grid;
  for (int r = 0; r < 3; ++r) {
    Vec<int>& row = grid.emplace_back();
    for (int c = 0; c <= r; ++c) row.push_back(r * 10 + c);
  }
  auto v = vec_of<long>(1, 2L, 'c', short(4));
  int sum = 0;
  for (const auto& row : grid)
    for (int x : row) sum += x;
  for (long x : v) sum += static_cast<int>(x);
  return sum == (0 + 10 + 11 + 20 + 21 + 22) + (1 + 2 + 99 + 4) ? 0 : 10;
}

int main() {
  if (int r = check_tracked()) return r;
  return check_nested();
}
