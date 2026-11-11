// tidal-rules: CPT-28
// tidal-expect: accept
// tidal-note: Ledger row P4-4. A parameter of a requires expression has the iterator type of a range, which decltype of a call to a function that is only declared names. No iterator is made. A type for which the alias is not valid makes the concept false. Every compiler accepts the program.
template <class T> T&& value_of();
template <class R> using IteratorOf = decltype(value_of<R&>().begin());

template <class R>
concept Sequence = requires(IteratorOf<R> it, R& range) {
  ++it;
  *it;
  bool(it != range.end());
};

struct Digits {
  struct Step {
    int i;
    int operator*() const { return i; }
    Step& operator++() { ++i; return *this; }
    bool operator!=(const Step& other) const { return i != other.i; }
    Step(int start) : i(start) {}
  };
  Step begin() { return {0}; }
  Step end() { return {10}; }
};
struct Bare { int begin() { return 0; } };

static_assert(Sequence<Digits>);
static_assert(!Sequence<int>);
static_assert(!Sequence<Bare>);
