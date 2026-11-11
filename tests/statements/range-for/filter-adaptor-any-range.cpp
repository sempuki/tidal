// tidal-rules: RFOR-2 RFOR-3
// tidal-expect: run
// tidal-note: Ledger row P4-3. An adaptor over any range keeps the elements that a closure accepts. The closure type is a template parameter, and aggregate deduction gives both parameters. decltype of a call to a declared function names the iterator type and the end type of the range. One range ends with != and the other with == alone. Every compiler accepts the program.
template <class T> T&& value_of();
template <class R> using IteratorOf = decltype(value_of<const R&>().begin());
template <class R> using EndOf = decltype(value_of<const R&>().end());

struct Done {};
struct Squares {
  int first, limit;
  struct Step {
    int i, limit;
    constexpr int operator*() const { return i * i; }
    constexpr Step& operator++() { ++i; return *this; }
    constexpr bool operator!=(Done) const { return i < limit; }
  };
  constexpr Step begin() const { return {first, limit}; }
  constexpr Done end() const { return {}; }
};
struct Count {
  int n;
  struct Step {
    int i, n;
    constexpr int operator*() const { return i; }
    constexpr Step& operator++() { ++i; return *this; }
    constexpr bool operator==(Done) const { return i == n; }
  };
  constexpr Step begin() const { return {0, n}; }
  constexpr Done end() const { return {}; }
};

template <class R, class Keep>
struct Filtered {
  R range;
  Keep keep;
  struct Step {
    IteratorOf<R> at;
    EndOf<R> last;
    Keep keep;
    constexpr void skip() { while (at != last && !keep(*at)) ++at; }
    constexpr auto operator*() const { return *at; }
    constexpr Step& operator++() { ++at; skip(); return *this; }
    constexpr bool operator!=(Done) const { return at != last; }
  };
  constexpr Step begin() const { Step s{range.begin(), range.end(), keep}; s.skip(); return s; }
  constexpr Done end() const { return {}; }
};

template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<IteratorOf<Squares>, Squares::Step>);
static_assert(same<EndOf<Count>, Done>);

constexpr int even_squares() {
  auto even = [](int v) { return v % 2 == 0; };
  int sum = 0;
  for (int sq : Filtered{Squares{1, 5}, even}) sum += sq;
  return sum;
}
constexpr int odd_counts() {
  auto odd = [](int v) { return v % 2 == 1; };
  int sum = 0;
  for (int v : Filtered{Count{7}, odd}) sum += v;
  return sum;
}
static_assert(even_squares() == 20);
static_assert(odd_counts() == 9);
int main() { return even_squares() == 20 && odd_counts() == 9 ? 0 : 1; }
