// A fixed-capacity vector, like a Rust ArrayVec<T, N>.
namespace std { using size_t = decltype(sizeof 0); }

template <class A, class B> struct same { static constexpr bool value = false; };
template <class A> struct same<A, A> { static constexpr bool value = true; };

template <class T, std::size_t N>
struct FixedVec {
  T items[N]{};
  std::size_t len = 0;

  constexpr bool push(const T& v) {
    if (len == N) return false;
    items[len++] = v;
    return true;
  }
  constexpr std::size_t size() const { return len; }

  // Like Rust's get(&self) -> Option<&T>: one body for const and non-const.
  constexpr auto* get(this auto& self, std::size_t i) {
    // nullptr alone is not a pointer (AUTO-5), so one return with ?: (COND).
    return i < self.len ? &self.items[i] : nullptr;
  }

  // One subscript for every kind of object.
  template <class Self>
  constexpr auto&& operator[](this Self&& self, std::size_t i) {
    return static_cast<Self&&>(self).items[i];
  }
};

template <class T, std::size_t R, std::size_t C>
struct Matrix {
  T cells[R * C]{};
  template <class Self>
  constexpr auto&& operator[](this Self&& self, std::size_t r, std::size_t c) {
    return static_cast<Self&&>(self).cells[r * C + c];
  }
};

constexpr int check() {
  FixedVec<int, 3> v;
  v.push(1); v.push(2); v.push(3);
  if (v.push(4)) return 1;
  v[1] = 20;
  *v.get(2) = 30;
  const FixedVec<int, 3>& cv = v;
  if (cv[1] != 20) return 2;
  if (cv.get(5) != nullptr) return 3;
  if (*cv.get(2) != 30) return 4;
  static_assert(same<decltype(cv[0]), const int&>::value);
  static_assert(same<decltype(v[0]), int&>::value);
  static_assert(same<decltype(cv.get(0)), const int*>::value);
  Matrix<int, 2, 3> m;
  m[1, 2] = 7;
  const auto& cm = m;
  if (cm[1, 2] != 7) return 5;
  static_assert(same<decltype(cm[0, 0]), const int&>::value);
  static_assert(same<decltype(Matrix<int, 1, 1>{}[0, 0]), int&&>::value);
  return 0;
}

static_assert(check() == 0);

int main() { return check(); }
