// A set of half-open intervals [lo, hi) of a fixed capacity, kept sorted
// and disjoint. Insertion merges, erasure splits, and union,
// intersection, and complement build new sets. A meeting planner finds
// free slots with it, in constant evaluation and at run time.
// No standard header.

using size_t = decltype(sizeof 0);

namespace iv {

template <class T>
struct Interval {
  T lo, hi;
  constexpr T length() const { return hi - lo; }
  constexpr bool operator==(const Interval&) const = default;
};

template <class T>
Interval(T, T) -> Interval<T>;

// The first position in [first, last) whose projection is not less
// than value.
template <class It, class V, class Proj>
constexpr It lower_bound(It first, It last, const V& value, Proj proj) {
  size_t n = last - first;
  while (n > 0) {
    size_t half = n / 2;
    It mid = first + half;
    if (proj(*mid) < value) {
      first = mid + 1;
      n -= half + 1;
    } else {
      n = half;
    }
  }
  return first;
}

template <class T, size_t N>
class IntervalSet {
 public:
  using value_type = Interval<T>;

  constexpr IntervalSet() = default;
  template <size_t M>
  constexpr IntervalSet(const value_type (&init)[M]) {
    for (const auto& [lo, hi] : init) insert(lo, hi);
  }

  constexpr size_t size() const { return n_; }
  constexpr const value_type* begin() const { return items_; }
  constexpr const value_type* end() const { return items_ + n_; }
  constexpr const value_type& operator[](size_t i) const { return items_[i]; }

  // Adds [lo, hi), merging every interval that overlaps or touches it.
  // Returns false when the set is full.
  constexpr bool insert(T lo, T hi) {
    if (!(lo < hi)) return true;
    value_type* first = lower_bound(items_, items_ + n_, lo, [](const value_type& x) { return x.hi; });
    value_type* last = first;
    while (last != items_ + n_ && !(hi < last->lo)) {
      if (last->lo < lo) lo = last->lo;
      if (hi < last->hi) hi = last->hi;
      ++last;
    }
    size_t removed = last - first;
    if (removed == 0) {
      if (n_ == N) return false;
      for (value_type* p = items_ + n_; p != first; --p) *p = p[-1];
      *first = {lo, hi};
      ++n_;
      return true;
    }
    *first = {lo, hi};
    shift_left(first + 1, removed - 1);
    return true;
  }

  // Removes [lo, hi). Returns false when a split needs room that the set
  // does not have.
  constexpr bool erase(T lo, T hi) {
    if (!(lo < hi)) return true;
    for (size_t i = 0; i < n_; ++i) {
      value_type& x = items_[i];
      if (!(x.lo < hi) || !(lo < x.hi)) continue;
      if (x.lo < lo && hi < x.hi) {
        if (n_ == N) return false;
        for (size_t j = n_; j > i + 1; --j) items_[j] = items_[j - 1];
        items_[i + 1] = {hi, x.hi};
        x.hi = lo;
        ++n_;
        return true;
      }
      if (x.lo < lo) {
        x.hi = lo;
      } else if (hi < x.hi) {
        x.lo = hi;
      } else {
        shift_left(items_ + i, 1);
        --i;
      }
    }
    return true;
  }

  constexpr bool contains(T v) const {
    const value_type* p = lower_bound(items_, items_ + n_, v, [](const value_type& x) { return x.hi; });
    if (p != items_ + n_ && p->hi == v) ++p;
    return p != items_ + n_ && !(v < p->lo);
  }

  constexpr T measure() const {
    T total{};
    for (const auto& x : *this) total += x.length();
    return total;
  }

  // The parts of [lo, hi) that the set does not cover.
  constexpr IntervalSet complement(T lo, T hi) const {
    IntervalSet out;
    out.insert(lo, hi);
    for (const auto& [a, b] : *this) out.erase(a, b);
    return out;
  }

  friend constexpr IntervalSet operator|(IntervalSet a, const IntervalSet& b) {
    for (const auto& [lo, hi] : b) a.insert(lo, hi);
    return a;
  }
  friend constexpr IntervalSet operator&(const IntervalSet& a, const IntervalSet& b) {
    IntervalSet out;
    size_t i = 0, j = 0;
    while (i < a.n_ && j < b.n_) {
      T lo = a[i].lo < b[j].lo ? b[j].lo : a[i].lo;
      T hi = a[i].hi < b[j].hi ? a[i].hi : b[j].hi;
      if (lo < hi) out.insert(lo, hi);
      if (a[i].hi < b[j].hi) ++i;
      else ++j;
    }
    return out;
  }
  friend constexpr bool operator==(const IntervalSet& a, const IntervalSet& b) {
    if (a.n_ != b.n_) return false;
    for (size_t i = 0; i < a.n_; ++i)
      if (!(a[i] == b[i])) return false;
    return true;
  }

 private:
  constexpr void shift_left(value_type* at, size_t by) {
    value_type* end = items_ + n_;
    for (value_type* p = at; p + by != end; ++p) *p = p[by];
    n_ -= by;
  }

  value_type items_[N]{};
  size_t n_ = 0;
};

}  // namespace iv

using Minutes = int;
using Set = iv::IntervalSet<Minutes, 8>;

constexpr Minutes at(int h, int m) { return h * 60 + m; }

// The earliest free slot of at least len minutes inside the working day,
// for every calendar at once, or -1.
template <size_t K>
constexpr Minutes first_free(const Set (&busy)[K], Minutes len) {
  Set all;
  for (const Set& s : busy) all = all | s;
  Set free = all.complement(at(9, 0), at(17, 0));
  for (auto [lo, hi] : free)
    if (hi - lo >= len) return lo;
  return -1;
}

constexpr bool set_test() {
  Set s({{1, 3}, {5, 8}, {3, 4}});
  if (s.size() != 2 || !(s[0] == iv::Interval{1, 4})) return false;
  if (!s.contains(1) || !s.contains(3) || s.contains(4) || s.contains(8) || !s.contains(7)) return false;
  s.insert(0, 10);
  if (s.size() != 1 || s.measure() != 10) return false;
  s.erase(2, 4);
  s.erase(6, 7);
  if (s.size() != 3 || s.measure() != 7) return false;
  Set t({{3, 9}});
  Set both = s & t;
  if (!(both == Set({{4, 6}, {7, 9}}))) return false;
  Set c = s.complement(-5, 15);
  return c == Set({{-5, 0}, {2, 4}, {6, 7}, {10, 15}});
}
static_assert(set_test());

constexpr Set alice({{at(9, 0), at(10, 30)}, {at(13, 0), at(14, 0)}});
constexpr Set bob({{at(10, 0), at(11, 0)}, {at(11, 30), at(12, 45)}});
constexpr Set busy[] = {alice, bob};
static_assert(first_free(busy, 30) == at(11, 0));
static_assert(first_free(busy, 45) == at(14, 0));
static_assert(first_free(busy, 4 * 60) == -1);

int main() {
  if (!set_test()) return 1;
  Set full;
  for (int i = 0; i < 8; ++i)
    if (!full.insert(i * 10, i * 10 + 5)) return 2;
  if (full.insert(100, 101)) return 3;   // no room
  if (!full.insert(5, 10)) return 4;     // merges two
  if (full.size() != 7) return 5;
  return first_free(busy, 60) == at(14, 0) ? 0 : 6;
}
