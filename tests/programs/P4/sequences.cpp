// tidal-rules: RFOR-2 RFOR-3 RFOR-7 OPR-4 OPR-13 LSUP-1 CTAD-9
// tidal-expect: run
// tidal-note: Port suite P4, Swift persona, program sequences.cpp (reports/clean-room/P4/report.md). The rules are the ones the report names for the program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// A port of Sequences.swift: a computed sequence with its own iterator,
// used by a range-based for and by lazy stages. Exits 0 when every
// check holds.

namespace std {
template <class E> class initializer_list {
  const E* first;
  decltype(sizeof 0) count;
  constexpr initializer_list(const E* f, decltype(sizeof 0) n)
      : first(f), count(n) {}
public:
  constexpr const E* begin() const { return first; }
  constexpr const E* end() const { return first + count; }
  constexpr decltype(sizeof 0) size() const { return count; }
};
}

// The end of every computed sequence (RFOR-2: end can have its own type).
struct Done {};

// A sequence is a type whose begin() and end() a range-based for can use.
template <class S>
concept Sequence = requires(const S& s) {
  s.begin() != s.end();
  *s.begin();
  ++*static_cast<decltype(s.begin())*>(nullptr);
};

template <class T> struct RemoveRef { using type = T; };
template <class T> struct RemoveRef<T&> { using type = T; };
template <class T> struct RemoveRef<T&&> { using type = T; };

template <class S> using IteratorOf = decltype(static_cast<const S*>(nullptr)->begin());
template <class S> using EndOf = decltype(static_cast<const S*>(nullptr)->end());
template <class S>
using ElementOf = typename RemoveRef<decltype(*static_cast<const S*>(nullptr)->begin())>::type;

// A growable list of fixed capacity, for the arrays of the Swift program.
template <class T, int Capacity>
struct List {
  T items[Capacity] = {};
  int count = 0;
  void append(T value) { items[count++] = value; }
  void removeFirst() {
    for (int i = 1; i < count; ++i) items[i - 1] = items[i];
    --count;
  }
  const T* begin() const { return items; }
  const T* end() const { return items + count; }
  bool matches(std::initializer_list<T> other) const {
    if (other.size() != static_cast<decltype(sizeof 0)>(count)) return false;
    int i = 0;
    for (const T& value : other)
      if (!(items[i++] == value)) return false;
    return true;
  }
};

template <class T> T maximum(T a, T b) { return a < b ? b : a; }

template <Sequence S, class T, class F>
T reduce(const S& sequence, T initial, F combine) {
  for (auto value : sequence) initial = combine(initial, value);
  return initial;
}

struct Collatz {
  long long start;

  struct Iterator {
    long long current;  // 0 when the sequence has ended
    long long operator*() const { return current; }
    Iterator& operator++() {
      if (current != 0)
        current = current == 1 ? 0 : (current % 2 == 0 ? current / 2 : 3 * current + 1);
      return *this;
    }
    bool operator==(Done) const { return current == 0; }
  };

  Iterator begin() const { return {start}; }
  Done end() const { return {}; }
};

template <Sequence S, class F>
struct Filtered {
  S base;
  F keep;

  struct Iterator {
    IteratorOf<S> at;
    EndOf<S> last;
    const F* keep;
    void skip() {
      while (at != last && !(*keep)(*at)) ++at;
    }
    ElementOf<S> operator*() const { return *at; }
    Iterator& operator++() {
      ++at;
      skip();
      return *this;
    }
    bool operator==(Done) const { return !(at != last); }
  };

  Iterator begin() const {
    Iterator first{base.begin(), base.end(), &keep};
    first.skip();
    return first;
  }
  Done end() const { return {}; }
};

template <Sequence S, class F>
struct Mapped {
  S base;
  F transform;

  struct Iterator {
    IteratorOf<S> at;
    EndOf<S> last;
    const F* transform;
    auto operator*() const { return (*transform)(*at); }
    Iterator& operator++() {
      ++at;
      return *this;
    }
    bool operator==(Done) const { return !(at != last); }
  };

  Iterator begin() const { return {base.begin(), base.end(), &transform}; }
  Done end() const { return {}; }
};

// The lazy view of a sequence, with stages that wrap it.
template <Sequence S>
struct Lazy {
  S base;
  auto begin() const { return base.begin(); }
  auto end() const { return base.end(); }
  template <class F> Lazy<Filtered<S, F>> filter(F keep) const { return {{base, keep}}; }
  template <class F> Lazy<Mapped<S, F>> map(F transform) const { return {{base, transform}}; }
};

template <Sequence S> Lazy<S> lazy(S base) { return {base}; }

// Groups of `size` neighbors of any sequence.
inline constexpr int windowCapacity = 16;

template <Sequence Base>
struct Window {
  Base base;
  int size;

  using Group = List<ElementOf<Base>, windowCapacity>;

  struct Iterator {
    IteratorOf<Base> inner;
    EndOf<Base> last;
    int size;
    Group buffer{};
    bool finished = false;
    void fill() {
      while (buffer.count < size) {
        if (!(inner != last)) {
          finished = true;
          return;
        }
        buffer.append(*inner);
        ++inner;
      }
    }
    Group operator*() const { return buffer; }
    Iterator& operator++() {
      buffer.removeFirst();
      fill();
      return *this;
    }
    bool operator==(Done) const { return finished; }
  };

  Iterator begin() const {
    Iterator first{base.begin(), base.end(), size};
    first.fill();
    return first;
  }
  Done end() const { return {}; }
};

int failures = 0;
void check(bool ok) {
  if (!ok) ++failures;
}

int main() {
  List<long long, 64> steps;
  for (long long value : Collatz{6}) steps.append(value);
  check(steps.matches({6, 3, 10, 5, 16, 8, 4, 2, 1}));

  auto evens = lazy(Collatz{27})
                   .filter([](long long v) { return v % 2 == 0; })
                   .map([](long long v) { return v / 2; });
  int count = 0;
  long long largest = 0;
  for (long long half : evens) {
    ++count;
    largest = maximum(largest, half);
  }
  check(count == 70);
  check(largest == 4616);

  List<long long, 64> sums;
  for (auto window : Window{Collatz{6}, 3})
    sums.append(reduce(window, 0LL, [](long long a, long long b) { return a + b; }));
  check(sums.matches({19, 18, 31, 29, 28, 14, 7}));

  auto it = Collatz{1}.begin();
  check(!(it == Done{}) && *it == 1);
  ++it;
  check(it == Done{});
  ++it;
  check(it == Done{});

  return failures == 0 ? 0 : 1;
}
