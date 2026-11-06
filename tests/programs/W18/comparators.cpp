// tidal-rules: OPR-1 OPR-2 OPR-3 CLS-15 OVL-25 MPTR-2 OVL-18
// tidal-expect: run
// tidal-note: Write suite W18, program comparators.cpp (reports/clean-room/W18/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Comparators as function objects, like Rust's Ord closures or Swift's
// areInIncreasingOrder parameter.
namespace std { using size_t = decltype(sizeof 0); }

struct Less {
  template <class T>
  constexpr bool operator()(const T& a, const T& b) const { return a < b; }
};

// No state, so the call operator can be static (CLS-15, OPR-3).
struct Greater {
  template <class T>
  static constexpr bool operator()(const T& a, const T& b) { return b < a; }
};

// A comparator with state: order by distance from a pivot.
struct ByDistance {
  int pivot;
  template <class T>
  constexpr bool operator()(const T& a, const T& b) const {
    auto da = a < pivot ? pivot - a : a - pivot;
    auto db = b < pivot ? pivot - b : b - pivot;
    return da < db;
  }
};

// Reverse any comparator, like Rust's Reverse or a Swift wrapper.
template <class C>
struct Reversed {
  C inner;
  template <class T>
  constexpr bool operator()(const T& a, const T& b) const { return inner(b, a); }
};

template <class T, std::size_t N, class Cmp>
constexpr void sort(T (&a)[N], Cmp cmp) {
  for (std::size_t i = 1; i < N; ++i) {
    T key = a[i];
    std::size_t j = i;
    while (j > 0 && cmp(key, a[j - 1])) {
      a[j] = a[j - 1];
      --j;
    }
    a[j] = key;
  }
}

template <class T, std::size_t N, class Cmp>
constexpr const T& max_by(const T (&a)[N], Cmp cmp) {
  const T* best = &a[0];
  for (const T& x : a)
    if (cmp(*best, x)) best = &x;
  return *best;
}

template <class T, std::size_t N>
constexpr bool equal(const T (&a)[N], const T (&b)[N]) {
  for (std::size_t i = 0; i < N; ++i)
    if (a[i] != b[i]) return false;
  return true;
}

constexpr int check() {
  int a[5] = {4, 1, 5, 2, 3};
  sort(a, Less{});
  const int up[5] = {1, 2, 3, 4, 5};
  if (!equal(a, up)) return 1;

  sort(a, Greater{});
  const int down[5] = {5, 4, 3, 2, 1};
  if (!equal(a, down)) return 2;

  sort(a, Reversed<Greater>{});
  if (!equal(a, up)) return 3;

  if (max_by(a, Less{}) != 5) return 4;
  if (max_by(a, Greater{}) != 1) return 5;
  if (max_by(a, ByDistance{3}) != 1) return 6;  // 1 and 5 tie; first wins

  // The static operator called with no object and through the class.
  if (!Greater::operator()(2, 1)) return 7;
  // Its address is an ordinary function pointer.
  bool (*gp)(const int&, const int&) = &Greater::operator()<int>;
  if (!gp(3, 2)) return 8;

  double d[3] = {2.5, -1.0, 0.5};
  sort(d, Less{});
  if (d[0] != -1.0 || d[2] != 2.5) return 9;
  return 0;
}

static_assert(check() == 0);

int main() { return check(); }
