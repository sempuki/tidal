// Ring<T, N>: a fixed-capacity circular queue whose slots are a union
// array in the object, so that each element starts its life when it is
// pushed. push_back, emplace_back, pop_front, front, back, operator[],
// a random-access iterator with <=> and arithmetic, copy and move,
// equality over two rings of different capacity, and constant tests.
namespace std {
using size_t = decltype(sizeof 0);
using ptrdiff_t = decltype(static_cast<int*>(nullptr) - static_cast<int*>(nullptr));
template <class T, class... Args>
constexpr T* construct_at(T* at, Args&&... args) {
  return ::new (static_cast<void*>(at)) T(static_cast<Args&&>(args)...);
}
struct strong_ordering {
  int v;
  static const strong_ordering less, equal, greater;
  friend constexpr bool operator==(strong_ordering a, strong_ordering b) = default;
  friend constexpr bool operator<(strong_ordering a, int) { return a.v < 0; }
  friend constexpr bool operator>(strong_ordering a, int) { return a.v > 0; }
  friend constexpr bool operator<=(strong_ordering a, int) { return a.v <= 0; }
  friend constexpr bool operator>=(strong_ordering a, int) { return a.v >= 0; }
  friend constexpr bool operator<(int, strong_ordering a) { return 0 < a.v; }
  friend constexpr bool operator>(int, strong_ordering a) { return 0 > a.v; }
  friend constexpr bool operator<=(int, strong_ordering a) { return 0 <= a.v; }
  friend constexpr bool operator>=(int, strong_ordering a) { return 0 >= a.v; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}
inline void* operator new(std::size_t, void* at) noexcept { return at; }

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };
template <class T> constexpr T&& forward(typename remove_ref<T>::type& t) noexcept { return static_cast<T&&>(t); }
template <class T> constexpr typename remove_ref<T>::type&& move(T&& t) noexcept {
  return static_cast<typename remove_ref<T>::type&&>(t);
}

struct Full {};
struct Empty {};

template <class T, std::size_t N>
class Ring {
  static_assert(N > 0);
  union Slot {
    T value;
    constexpr Slot() {}
    constexpr ~Slot() {}
  };
  Slot slots_[N];
  std::size_t head_ = 0;
  std::size_t size_ = 0;

  constexpr std::size_t at(std::size_t i) const { return (head_ + i) % N; }

 public:
  template <bool Const>
  class Iter {
    // Fifth try: the other Iter is a friend, since the conversion reads
    // its members, and the fourth try lost the access that
    // `friend class Ring;` gave every member of Ring (ACC-5).
    template <bool> friend class Iter;
    const Ring* r_;
    std::size_t i_;

   public:
    // Fourth try: no friend declaration and a public constructor, since
    // the front end reads `friend class Ring;` as a new class (TYN-4).
    constexpr Iter(const Ring* r, std::size_t i) : r_(r), i_(i) {}
    using difference_type = std::ptrdiff_t;
    constexpr Iter() : r_(nullptr), i_(0) {}
    constexpr auto& operator*() const {
      if constexpr (Const) return r_->slots_[r_->at(i_)].value;
      else return const_cast<Ring*>(r_)->slots_[r_->at(i_)].value;
    }
    constexpr auto& operator[](difference_type n) const { return *(*this + n); }
    // Third try: operator->, which main uses.
    constexpr auto* operator->() const { return &**this; }
    constexpr Iter& operator++() { ++i_; return *this; }
    constexpr Iter operator++(int) { Iter t = *this; ++i_; return t; }
    constexpr Iter& operator--() { --i_; return *this; }
    constexpr Iter& operator+=(difference_type n) { i_ += n; return *this; }
    constexpr friend Iter operator+(Iter it, difference_type n) { return it += n; }
    constexpr friend Iter operator+(difference_type n, Iter it) { return it += n; }
    constexpr friend difference_type operator-(Iter a, Iter b) {
      return static_cast<difference_type>(a.i_) - static_cast<difference_type>(b.i_);
    }
    constexpr friend bool operator==(Iter a, Iter b) { return a.i_ == b.i_; }
    constexpr friend std::strong_ordering operator<=>(Iter a, Iter b) {
      return a.i_ < b.i_ ? std::strong_ordering::less
           : a.i_ > b.i_ ? std::strong_ordering::greater
                         : std::strong_ordering::equal;
    }
    // A const iterator from a mutable one. Second try: a template, since a
    // constrained non-template Iter(const Iter<false>&) is the copy
    // constructor of Iter<false> and suppresses the implicit one.
    template <bool C = Const>
      requires C
    constexpr Iter(const Iter<false>& o) : r_(o.r_), i_(o.i_) {}
  };
  using iterator = Iter<false>;
  using const_iterator = Iter<true>;

  constexpr Ring() = default;
  constexpr Ring(const Ring& o) : head_(0), size_(0) {
    for (const T& v : o) push_back(v);
  }
  constexpr Ring(Ring&& o) noexcept : head_(0), size_(0) {
    for (T& v : o) push_back(move(v));
    o.clear();
  }
  constexpr Ring& operator=(Ring o) noexcept {
    clear();
    for (T& v : o) push_back(move(v));
    return *this;
  }
  constexpr ~Ring() { clear(); }

  constexpr std::size_t size() const { return size_; }
  constexpr bool empty() const { return size_ == 0; }
  constexpr bool full() const { return size_ == N; }
  static constexpr std::size_t capacity() { return N; }

  template <class... Args>
  constexpr T& emplace_back(Args&&... args) {
    if (full()) throw Full{};
    T* p = std::construct_at(&slots_[at(size_)].value, forward<Args>(args)...);
    ++size_;
    return *p;
  }
  constexpr void push_back(const T& v) { emplace_back(v); }
  constexpr void push_back(T&& v) { emplace_back(move(v)); }
  // Pushes and drops the oldest element when full.
  constexpr void push_overwrite(T v) {
    if (full()) pop_front();
    emplace_back(move(v));
  }
  constexpr T pop_front() {
    if (empty()) throw Empty{};
    T& slot = slots_[head_].value;
    T out = move(slot);
    slot.~T();
    head_ = (head_ + 1) % N;
    --size_;
    return out;
  }
  constexpr void clear() {
    while (size_ > 0) {
      slots_[head_].value.~T();
      head_ = (head_ + 1) % N;
      --size_;
    }
    head_ = 0;
  }
  constexpr T& front() { return slots_[head_].value; }
  constexpr T& back() { return slots_[at(size_ - 1)].value; }
  constexpr T& operator[](std::size_t i) { return slots_[at(i)].value; }
  constexpr const T& operator[](std::size_t i) const { return slots_[at(i)].value; }

  constexpr iterator begin() { return {this, 0}; }
  constexpr iterator end() { return {this, size_}; }
  constexpr const_iterator begin() const { return {this, 0}; }
  constexpr const_iterator end() const { return {this, size_}; }
};

template <class T, std::size_t A, std::size_t B>
constexpr bool operator==(const Ring<T, A>& a, const Ring<T, B>& b) {
  if (a.size() != b.size()) return false;
  auto j = b.begin();
  for (const T& x : a) if (!(x == *j++)) return false;
  return true;
}

template <class It>
constexpr It lower_bound(It first, It last, const auto& v) {
  auto n = last - first;
  while (n > 0) {
    auto half = n / 2;
    It mid = first + half;
    if (*mid < v) { first = mid + 1; n -= half + 1; }
    else n = half;
  }
  return first;
}

constexpr int constant_test() {
  Ring<int, 4> r; r.push_back(1); int s = 0; for (int v : r) s += v; return s - 1;
}
static_assert(constant_test() == 0);

inline int live = 0;
struct Tracked {
  int id;
  explicit Tracked(int i) : id(i) { ++live; }
  Tracked(const Tracked& o) : id(o.id) { ++live; }
  Tracked(Tracked&& o) noexcept : id(o.id) { o.id = -1; ++live; }
  ~Tracked() { --live; }
  friend bool operator==(const Tracked& a, const Tracked& b) { return a.id == b.id; }
};

int main() {
  int fails = 0;
  
  {
    Ring<Tracked, 3> r;
    r.emplace_back(1);
    r.emplace_back(2);
    r.emplace_back(3);
    if (live != 3) ++fails;
    try { r.emplace_back(4); ++fails; } catch (Full) {}
    if (live != 3) ++fails;
    Tracked t = r.pop_front();
    if (t.id != 1 || live != 3) ++fails;
    r.push_overwrite(Tracked(4));
    r.push_overwrite(Tracked(5));  // drops 2
    if (r.front().id != 3 || r.back().id != 5 || live != 4) ++fails;
    Ring<Tracked, 3> c = r;
    if (!(c == r) || live != 7) ++fails;
    Ring<Tracked, 3>::const_iterator ci = c.begin();
    if ((ci + 2)->id != 5) ++fails;
    if (!(ci < c.end()) || c.end() - ci != 3) ++fails;
    c = Ring<Tracked, 3>();
    if (!c.empty() || live != 4) ++fails;
  }
  if (live != 0) ++fails;
  {
    Ring<int, 2> r;
    try { r.pop_front(); ++fails; } catch (Empty) {}
  }
  return fails;
}
