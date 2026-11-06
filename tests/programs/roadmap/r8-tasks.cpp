// tidal-rules: PACK-14 CTAD-1 LAM-12 CLS-13 THR-6
// tidal-expect: run
// tidal-note: Roadmap round 8, programmer sketch tasks.cpp (reports/clean-room/roadmap-8/programmer.md, section 1.4). The rules are the ones the report names for this program and its reductions, and the rules of the main constructs it uses.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Deferred tasks: a tuple built by recursive inheritance with a
// deduction guide, an index sequence and apply, lambdas that capture a
// pack with an init-capture, a Result<T> with and_then and transform
// through a deduced object parameter, and an exception class hierarchy
// with a rethrow.
namespace std {
using size_t = decltype(sizeof 0);
class exception {
public:
  virtual ~exception() = default;
  virtual const char* what() const noexcept { return "exception"; }
};
}

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };
template <class T> using remove_ref_t = typename remove_ref<T>::type;
template <class T> constexpr T&& forward(remove_ref_t<T>& t) noexcept { return static_cast<T&&>(t); }
template <class T> constexpr remove_ref_t<T>&& move(T&& t) noexcept { return static_cast<remove_ref_t<T>&&>(t); }

template <std::size_t... I> struct index_sequence {};
template <std::size_t N, std::size_t... I>
struct make_seq : make_seq<N - 1, N - 1, I...> {};
template <std::size_t... I>
struct make_seq<0, I...> { using type = index_sequence<I...>; };
template <std::size_t N> using make_index_sequence = typename make_seq<N>::type;

template <std::size_t I, class T> struct leaf { T value; };

template <class Seq, class... T> struct tuple_base;
template <std::size_t... I, class... T>
struct tuple_base<index_sequence<I...>, T...> : leaf<I, T>... {
  constexpr tuple_base(T... v) : leaf<I, T>{v}... {}
};

template <class... T>
struct Tuple : tuple_base<make_index_sequence<sizeof...(T)>, T...> {
  using tuple_base<make_index_sequence<sizeof...(T)>, T...>::tuple_base;
  static constexpr std::size_t size = sizeof...(T);
};
template <class... T> Tuple(T...) -> Tuple<T...>;

template <std::size_t I, class T> constexpr T& get(leaf<I, T>& l) { return l.value; }
template <std::size_t I, class T> constexpr const T& get(const leaf<I, T>& l) { return l.value; }

template <class F, class Tup, std::size_t... I>
constexpr decltype(auto) apply_impl(F&& f, Tup& t, index_sequence<I...>) {
  return forward<F>(f)(get<I>(t)...);
}
template <class F, class Tup>
constexpr decltype(auto) apply(F&& f, Tup& t) {
  return apply_impl(forward<F>(f), t, make_index_sequence<Tup::size>{});
}

template <class F, class... Args>
constexpr auto defer(F f, Args&&... args) {
  return [f, ...xs = forward<Args>(args)]() mutable { return f(xs...); };
}

struct Error {
  int code;
};

template <class T>
class [[nodiscard]] Result {
  bool ok_;
  union { T value_; Error error_; };
public:
  constexpr Result(T v) : ok_(true), value_(v) {}
  constexpr Result(Error e) : ok_(false), error_(e) {}
  constexpr bool ok() const { return ok_; }
  constexpr int error() const { return ok_ ? 0 : error_.code; }
  template <class Self, class F>
  constexpr auto and_then(this Self&& self, F f) -> decltype(f(self.value_)) {
    if (self.ok_) return f(forward<Self>(self).value_);
    return self.error_;
  }
  template <class Self, class F>
  constexpr auto transform(this Self&& self, F f) -> Result<decltype(f(self.value_))> {
    if (self.ok_) return f(forward<Self>(self).value_);
    return self.error_;
  }
  constexpr T value_or(T d) const { return ok_ ? value_ : d; }
};

constexpr Result<int> parse_digit(char c) {
  if (c >= '0' && c <= '9') return c - '0';
  return Error{1};
}
constexpr Result<int> halve(int v) {
  if (v % 2) return Error{2};
  return v / 2;
}

static_assert(parse_digit('8').and_then(halve).transform([](int v) { return v * 10; }).value_or(-1) == 40);
static_assert(parse_digit('7').and_then(halve).error() == 2);
static_assert(parse_digit('x').and_then(halve).error() == 1);

class TaskError : public std::exception {
  int id_;
public:
  explicit TaskError(int id) : id_(id) {}
  const char* what() const noexcept override { return "task failed"; }
  int id() const { return id_; }
};

int run_task(int id) {
  if (id == 3) throw TaskError(id);
  return id * 2;
}

int run_all(int& failures) {
  int sum = 0;
  for (int id = 1; id <= 4; ++id) {
    try {
      try {
        sum += run_task(id);
      } catch (const TaskError& e) {
        failures += e.id();
        throw;
      }
    } catch (const std::exception& e) {
      if (e.what()[0] != 't') return -1;
    }
  }
  return sum;
}

int main() {
  constexpr Tuple t{1, 'x', 2L};
  static_assert(get<2>(t) == 2L && t.size == 3);
  Tuple u{3, 4, 5};
  int prod = apply([](int a, int b, int c) { return a * b * c; }, u);
  if (prod != 60) return 1;

  int base = 10;
  auto task = defer([base](int a, int b) { return base + a + b; }, 1, 2);
  if (task() != 13) return 2;

  int failures = 0;
  if (run_all(failures) != 2 + 4 + 8) return 3;
  if (failures != 3) return 4;
  return 0;
}
