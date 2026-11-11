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
};
}
inline void* operator new(std::size_t, void* at) noexcept { return at; }
template <class T> struct remove_ref { using type = T; };
template <class T> constexpr T&& forward(typename remove_ref<T>::type& t) noexcept { return static_cast<T&&>(t); }
template <class T> constexpr typename remove_ref<T>::type&& move(T&& t) noexcept {
  return static_cast<typename remove_ref<T>::type&&>(t);
}
struct Full {};
struct Empty {};
template <class T, std::size_t N>
class Ring {
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
    using R = Ring;
    friend class Ring;
    const Ring* r_;
    std::size_t i_;
    constexpr Iter(const Ring* r, std::size_t i) : r_(r), i_(i) {}
   public:
    using difference_type = std::ptrdiff_t;
    constexpr auto& operator*() const {
      if constexpr (Const) return r_->slots_[r_->at(i_)].value;
      else return const_cast<Ring*>(r_)->slots_[r_->at(i_)].value;
    }
    constexpr Iter& operator++() { ++i_; return *this; }
    constexpr Iter& operator+=(difference_type n) { i_ += n; return *this; }
    constexpr friend Iter operator+(Iter it, difference_type n) { return it += n; }
    constexpr friend difference_type operator-(Iter a, Iter b) {
      return static_cast<difference_type>(a.i_) - static_cast<difference_type>(b.i_);
    }
    constexpr friend bool operator==(Iter a, Iter b) { return a.i_ == b.i_; }
    constexpr friend std::strong_ordering operator<=>(Iter a, Iter b) {
      return a.i_ < b.i_ ? std::strong_ordering::less
                         : std::strong_ordering::equal;
    }
  };
  using iterator = Iter<false>;
  using const_iterator = Iter<true>;
  constexpr Ring() = default;
  constexpr Ring(Ring&& o) noexcept : head_(0), size_(0) {
  }
  constexpr Ring& operator=(Ring o) noexcept {
  }
  constexpr bool empty() const { return size_ == 0; }
  constexpr bool full() const { return size_ == N; }
  template <class... Args>
  constexpr T& emplace_back(Args&&... args) {
    if (full()) throw Full{};
    T* p = std::construct_at(&slots_[at(size_)].value, forward<Args>(args)...);
    ++size_;
    return *p;
  }
  constexpr void push_back(const T& v) { emplace_back(v); }
  constexpr void push_overwrite(T v) {
    if (full()) pop_front();
    emplace_back(move(v));
  }
  constexpr T pop_front() {
    T& slot = slots_[head_].value;
    T out = move(slot);
    head_ = (head_ + 1) % N;
    --size_;
    return out;
  }
  constexpr void clear() {
    while (size_ > 0) {
    }
  }
  constexpr iterator begin() { return {this, 0}; }
  constexpr iterator end() { return {this, size_}; }
};
template <class T, std::size_t A, std::size_t B>
constexpr bool operator==(const Ring<T, A>& a, const Ring<T, B>& b) {
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
  Ring<int, 4> r;
  for (int i = 1; i <= 6; ++i) r.push_overwrite(i * 10);  // keeps 30 40 50 60
  if (r.pop_front() != 30) return 2;
  auto it = lower_bound(r.begin(), r.end(), 55);
  Ring<int, 8> copy;
  return 0;
}
inline int live = 0;
struct Tracked {
  int id;
  explicit Tracked(int i) : id(i) { ++live; }
};
int main() {
  int fails = 0;
  {
    Ring<Tracked, 3> r;
    r.emplace_back(1);
    r.emplace_back(2);
    r.emplace_back(3);
  }
}