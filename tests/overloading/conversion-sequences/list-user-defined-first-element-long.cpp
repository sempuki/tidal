// tidal-rules: ICS-16 ICS-25
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: ambiguous
// tidal-note: Q-0423. Decision 0059, item F2 (I3-13). The first element converts by Q to long and the second by P to int. The list sequence holds the first element's conversion, so the long form wins. Clang and EDG choose it; GCC calls the call ambiguous.
namespace std {
template <class E> class initializer_list {
  const E* first_;
  decltype(sizeof 0) size_;
  constexpr initializer_list(const E* f, decltype(sizeof 0) n) : first_(f), size_(n) {}
public:
  constexpr initializer_list() : first_(nullptr), size_(0) {}
};
}
struct P { constexpr operator int() const { return 1; } };
struct Q { constexpr operator long() const { return 2; } };
constexpr int f(std::initializer_list<int>) { return 1; }
constexpr int f(std::initializer_list<long>) { return 2; }
static_assert(f({Q{}, P{}}) == 2);
