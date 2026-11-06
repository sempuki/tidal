// tidal-rules: PACK-17 CPT-1 VIRT-1 MPTR-3 INIT-9 CTOR-2
// tidal-expect: run
// tidal-note: Roadmap round 9, programmer sketch function.cpp (reports/clean-room/roadmap-9/programmer.md, section 1.4). The rules are the ones the report names for this program and its reductions, and the rules of the main constructs it uses.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// A type-erased callable: Function<R(Args...)> as a partial specialization
// on a function type, a constructor template constrained against its own
// type, storage through a virtual interface with clone, a call operator that
// forwards its parameters, bind_front with an init-capture pack, an invoke
// that handles pointers to members, and a small event list of handlers.
namespace std { using size_t = decltype(sizeof 0); }

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };
template <class T> using remove_ref_t = typename remove_ref<T>::type;
template <class T> struct remove_cvref { using type = T; };
template <class T> struct remove_cvref<const T> { using type = T; };
template <class T> struct remove_cvref<T&> : remove_cvref<T> {};
template <class T> struct remove_cvref<T&&> : remove_cvref<T> {};
template <class T> using remove_cvref_t = typename remove_cvref<T>::type;
template <class T> constexpr remove_ref_t<T>&& move(T&& t) noexcept { return static_cast<remove_ref_t<T>&&>(t); }
template <class T> constexpr T&& forward(remove_ref_t<T>& t) noexcept { return static_cast<T&&>(t); }
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;

template <class T> struct is_member_pointer_t { static constexpr bool value = false; };
template <class M, class C> struct is_member_pointer_t<M C::*> { static constexpr bool value = true; };

template <class F, class Obj, class... A>
constexpr decltype(auto) invoke(F&& f, Obj&& obj, A&&... a) {
  if constexpr (is_member_pointer_t<remove_cvref_t<F>>::value)
    return (forward<Obj>(obj).*f)(forward<A>(a)...);
  else
    return forward<F>(f)(forward<Obj>(obj), forward<A>(a)...);
}

template <class Sig> class Function;

template <class R, class... Args>
class Function<R(Args...)> {
  struct Concept {
    virtual ~Concept() = default;
    virtual R call(Args... args) = 0;
    virtual Concept* clone() const = 0;
  };
  template <class F>
  struct Model final : Concept {
    F f;
    explicit Model(F&& g) : f(move(g)) {}
    explicit Model(const F& g) : f(g) {}
    R call(Args... args) override { return invoke(f, static_cast<Args&&>(args)...); }
    Concept* clone() const override { return new Model(f); }
  };
  Concept* p_ = nullptr;

public:
  Function() = default;
  template <class F>
    requires(!same<remove_cvref_t<F>, Function>)
  Function(F&& f) : p_(new Model<remove_cvref_t<F>>(forward<F>(f))) {}
  Function(const Function& o) : p_(o.p_ ? o.p_->clone() : nullptr) {}
  Function(Function&& o) noexcept : p_(o.p_) { o.p_ = nullptr; }
  Function& operator=(Function o) noexcept {
    Concept* t = p_; p_ = o.p_; o.p_ = t;
    return *this;
  }
  ~Function() { delete p_; }
  explicit operator bool() const { return p_ != nullptr; }
  R operator()(Args... args) const { return p_->call(static_cast<Args&&>(args)...); }
};

template <class F, class... X>
auto bind_front(F&& f, X&&... xs) {
  return [f = forward<F>(f), ... xs = forward<X>(xs)](auto&&... rest) mutable {
    return invoke(f, xs..., forward<decltype(rest)>(rest)...);
  };
}

struct Counter {
  int total = 0;
  int add(int x) { total += x; return total; }
};

struct Event {
  static constexpr std::size_t max = 4;
  Function<void(int&)> handlers[max];
  std::size_t n = 0;
  template <class H> void on(H&& h) { handlers[n++] = Function<void(int&)>(forward<H>(h)); }
  void fire(int& acc) const {
    for (std::size_t i = 0; i < n; ++i) handlers[i](acc);
  }
};

int square(int x) { return x * x; }

int main() {
  Function<int(int)> f = &square;   // a function would deduce a function type
  if (f(5) != 25) return 1;
  int base = 10;
  f = Function<int(int)>([base](int x) { return base + x; });
  if (f(5) != 15) return 2;
  Function<int(int)> g = f;          // clone
  f = Function<int(int)>([](int x) { return -x; });
  if (g(1) != 11 || f(1) != -1) return 3;

  Counter c;
  Function<int(Counter&, int)> m = &Counter::add;
  m(c, 3);
  m(c, 4);
  if (c.total != 7) return 4;

  auto add_to_c = bind_front(&Counter::add, &c);   // a Counter* bound first
  (void)add_to_c;
  auto plus = bind_front([](int a, int b, int d) { return a * 100 + b * 10 + d; }, 1, 2);
  if (plus(3) != 123) return 5;

  Event e;
  e.on([](int& a) { a += 1; });
  e.on([k = 5](int& a) { a *= k; });
  int acc = 1;
  e.fire(acc);
  return acc == 10 ? 0 : 6;
}
