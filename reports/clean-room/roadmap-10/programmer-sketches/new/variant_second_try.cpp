// A Variant<Ts...> over a recursive union, with an index, a converting
// constructor that picks the alternative by exact type, emplace, copy and
// move that dispatch on the index through a table of function pointers,
// visit with an overload set built from lambdas, holds_alternative and
// get_if, and an alternative with a nontrivial destructor.
namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* p) noexcept { return p; }

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };
template <class T> using remove_ref_t = typename remove_ref<T>::type;
template <class T> struct remove_c { using type = T; };
template <class T> struct remove_c<const T> { using type = T; };
template <class T> using plain_t = typename remove_c<remove_ref_t<T>>::type;
template <class T> constexpr T&& forward(remove_ref_t<T>& t) noexcept { return static_cast<T&&>(t); }
template <class T> constexpr remove_ref_t<T>&& move(T&& t) noexcept { return static_cast<remove_ref_t<T>&&>(t); }

template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;

template <class T, class... Ts> constexpr std::size_t index_of = 0;
template <class T, class U, class... Ts>
constexpr std::size_t index_of<T, U, Ts...> = same<T, U> ? 0 : 1 + index_of<T, Ts...>;

template <std::size_t I, class... Ts> struct nth;
template <class T, class... Ts> struct nth<0, T, Ts...> { using type = T; };
template <std::size_t I, class T, class... Ts> struct nth<I, T, Ts...> : nth<I - 1, Ts...> {};

template <class... Fs> struct overloaded : Fs... { using Fs::operator()...; };

template <class... Ts> union Storage {
  constexpr Storage() {}
};
template <class T, class... Ts> union Storage<T, Ts...> {
  T head;
  Storage<Ts...> tail;
  constexpr Storage() {}
  constexpr ~Storage() {}
};
template <class T, class S> constexpr T& slot(S& s) {
  if constexpr (requires { requires same<decltype(s.head), T>; }) return s.head;
  else return slot<T>(s.tail);
}
template <class T, class S> constexpr const T& slot(const S& s) {
  if constexpr (requires { requires same<decltype(s.head), T>; }) return s.head;
  else return slot<T>(s.tail);
}

// Second version: the alternatives live in a recursive union, so each
// alternative is a member and no cast from the byte array is needed.
template <class... Ts> class Variant {
  Storage<Ts...> storage_;
  std::size_t index_ = 0;

  template <class T> T& as() noexcept { return slot<T>(storage_); }
  template <class T> const T& as() const noexcept { return slot<T>(storage_); }
  template <class T, class... A> void build(A&&... a) {
    ::new (static_cast<void*>(&as<T>())) T(forward<A>(a)...);
  }

  template <class T> static void destroy_one(Variant& v) noexcept { v.as<T>().~T(); }
  template <class T> static void copy_one(Variant& to, const Variant& from) { to.build<T>(from.as<T>()); }
  template <class T> static void move_one(Variant& to, Variant& from) { to.build<T>(move(from.as<T>())); }
  void destroy() noexcept {
    using Fn = void (*)(Variant&) noexcept;
    static constexpr Fn table[] = {&destroy_one<Ts>...};
    table[index_](*this);
  }
  void move_from(Variant& o) {
    using Fn = void (*)(Variant&, Variant&);
    static constexpr Fn table[] = {&move_one<Ts>...};
    table[index_](*this, o);
  }

public:
  Variant() : Variant(typename nth<0, Ts...>::type()) {}
  template <class U> requires (index_of<plain_t<U>, Ts...> < sizeof...(Ts))
  Variant(U&& u) : index_(index_of<plain_t<U>, Ts...>) { build<plain_t<U>>(forward<U>(u)); }
  Variant(const Variant& o) : index_(o.index_) {
    using Fn = void (*)(Variant&, const Variant&);
    static constexpr Fn table[] = {&copy_one<Ts>...};
    table[index_](*this, o);
  }
  Variant(Variant&& o) : index_(o.index_) { move_from(o); }
  Variant& operator=(const Variant& o) {
    if (this != &o) { Variant tmp(o); destroy(); index_ = o.index_; move_from(tmp); }
    return *this;
  }
  ~Variant() { destroy(); }

  template <class T, class... A> T& emplace(A&&... a) {
    destroy();
    index_ = index_of<T, Ts...>;
    build<T>(forward<A>(a)...);
    return as<T>();
  }
  std::size_t index() const noexcept { return index_; }
  template <class T> bool holds() const noexcept { return index_ == index_of<T, Ts...>; }
  template <class T> T* get_if() noexcept { return holds<T>() ? &as<T>() : nullptr; }

  template <class F> decltype(auto) visit(F&& f) {
    using R = decltype(f(as<typename nth<0, Ts...>::type>()));
    using Fn = R (*)(Variant&, F&);
    static constexpr Fn table[] = {[](Variant& v, F& g) -> R { return g(v.as<Ts>()); }...};
    return table[index_](*this, f);
  }
};

inline int live_names = 0;
struct Name {
  char text[8] = {};
  Name(const char* s) { for (int i = 0; i < 7 && s[i]; ++i) text[i] = s[i]; ++live_names; }
  Name(const Name& o) { for (int i = 0; i < 8; ++i) text[i] = o.text[i]; ++live_names; }
  ~Name() { --live_names; }
  int length() const { int n = 0; while (text[n]) ++n; return n; }
};

int main() {
  int fails = 0;
  {
    using V = Variant<int, double, Name>;
    V a = 3;
    V b = 2.5;
    V c = Name("alice");
    if (a.index() != 0 || b.index() != 1 || c.index() != 2) ++fails;
    auto size = overloaded{
      [](int i) { return i; },
      [](double d) { return static_cast<int>(d * 2); },
      [](const Name& n) { return n.length(); },
    };
    if (a.visit(size) != 3 || b.visit(size) != 5 || c.visit(size) != 5) ++fails;
    V d = c;
    if (live_names != 2 || !d.holds<Name>()) ++fails;
    d = a;
    if (live_names != 1 || *d.get_if<int>() != 3 || d.get_if<double>()) ++fails;
    c.emplace<double>(1.5);
    if (live_names != 0 || *c.get_if<double>() != 1.5) ++fails;
    V e;
    if (e.index() != 0 || *e.get_if<int>() != 0) ++fails;
    e.emplace<Name>("bob");
    V f = move(e);
    if (live_names != 2 || f.visit(size) != 3) ++fails;
  }
  if (live_names != 0) ++fails;
  return fails;
}
