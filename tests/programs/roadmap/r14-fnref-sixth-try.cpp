// tidal-rules: LAM-5 LAM-18 CTOR-10 ICS-9
// tidal-expect: run
// tidal-note: Roadmap round 14, programmer sketch fnref_sixth_try.cpp (reports/clean-room/roadmap-14/programmer.md, section 1.4). The sixth try builds clean on GCC and Clang, so it is a program test (decision 0061). It differs from the last version in closures passed straight to FunctionRef parameters, in place of a conversion to FunctionRef by name. The rules are the ones the report names for this program and its reductions.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// A non-owning callable reference, FunctionRef<R(Args...)>: an object
// pointer and a thunk made from a function template specialization. It
// binds lambdas, function objects, plain functions, and pointers to
// members. A small text API takes callbacks through it. No standard header.

using size_t = decltype(sizeof 0);

namespace lib {

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };
template <class T> using remove_ref_t = typename remove_ref<T>::type;

template <class T> struct remove_cv { using type = T; };
template <class T> struct remove_cv<const T> { using type = T; };

template <class T> using remove_cvref_t = typename remove_cv<remove_ref_t<T>>::type;

template <class A, class B> inline constexpr bool same = false;
template <class A> inline constexpr bool same<A, A> = true;

template <class T> T&& declval() noexcept;

template <class T> inline constexpr bool is_member_pointer = false;
template <class M, class C> inline constexpr bool is_member_pointer<M C::*> = true;

template <class T>
constexpr T&& forward(remove_ref_t<T>& t) noexcept { return static_cast<T&&>(t); }

// invoke for function objects and pointers to members, with an object
// reference or a pointer as the first argument.
template <class F, class... A>
constexpr decltype(auto) invoke(F&& f, A&&... a) {
  if constexpr (is_member_pointer<remove_cvref_t<F>>) {
    return [f]<class O, class... Rest>(O&& o, Rest&&... rest) -> decltype(auto) {
      if constexpr (requires { (forward<O>(o).*f)(forward<Rest>(rest)...); })
        return (forward<O>(o).*f)(forward<Rest>(rest)...);
      else if constexpr (requires { ((*forward<O>(o)).*f)(forward<Rest>(rest)...); })
        return ((*forward<O>(o)).*f)(forward<Rest>(rest)...);
      else if constexpr (requires { forward<O>(o).*f; })
        return (forward<O>(o).*f);
      else
        return ((*forward<O>(o)).*f);
    }(forward<A>(a)...);
  } else {
    return forward<F>(f)(forward<A>(a)...);
  }
}

template <class From, class To>
concept convertible_to = requires(From (&f)()) { static_cast<To>(f()); };

template <class F, class R, class... A>
concept invocable_r = requires(F&& f, A&&... a) {
  { lib::invoke(forward<F>(f), forward<A>(a)...) } -> convertible_to<R>;
};

}  // namespace lib

namespace lib {

template <class Sig> class FunctionRef;

template <class R, class... A>
class FunctionRef<R(A...)> {
 public:
  template <class F>
    requires (!same<remove_cvref_t<F>, FunctionRef>) && invocable_r<F&, R, A...>
  constexpr FunctionRef(F&& f) noexcept
      : obj_(const_cast<void*>(static_cast<const void*>(&f))),
        call_(&thunk<remove_ref_t<F>>) {}

  constexpr FunctionRef(R (*fp)(A...)) noexcept : fp_(fp), call_(&fn_thunk) {}

  constexpr R operator()(A... a) const { return call_(*this, forward<A>(a)...); }

 private:
  template <class F>
  static constexpr R thunk(const FunctionRef& self, A... a) {
    return static_cast<R>(lib::invoke(*static_cast<F*>(self.obj_), forward<A>(a)...));
  }
  static constexpr R fn_thunk(const FunctionRef& self, A... a) {
    return self.fp_(forward<A>(a)...);
  }

  void* obj_ = nullptr;
  R (*fp_)(A...) = nullptr;
  R (*call_)(const FunctionRef&, A...);
};

}  // namespace lib

namespace text {

struct View {
  const char* p;
  size_t n;
  constexpr char operator[](size_t i) const { return p[i]; }
  constexpr bool operator==(const View& o) const {
    if (n != o.n) return false;
    for (size_t i = 0; i < n; ++i)
      if (p[i] != o.p[i]) return false;
    return true;
  }
};

constexpr size_t length(const char* s) {
  size_t n = 0;
  while (s[n]) ++n;
  return n;
}

constexpr View view(const char* s) { return {s, length(s)}; }

// Calls each(word) for each run of characters that is not sep.
constexpr int split(View v, char sep, lib::FunctionRef<void(View)> each) {
  int count = 0;
  size_t i = 0;
  while (i < v.n) {
    while (i < v.n && v[i] == sep) ++i;
    size_t start = i;
    while (i < v.n && v[i] != sep) ++i;
    if (i > start) {
      each(View{v.p + start, i - start});
      ++count;
    }
  }
  return count;
}

constexpr size_t count_if(View v, lib::FunctionRef<bool(char)> pred) {
  size_t c = 0;
  for (size_t i = 0; i < v.n; ++i)
    if (pred(v[i])) ++c;
  return c;
}

}  // namespace text

constexpr bool is_vowel(char c) {
  return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
}

struct Counter {
  int calls = 0;
  size_t longest = 0;
  constexpr void operator()(text::View w) {
    ++calls;
    if (w.n > longest) longest = w.n;
  }
};

struct Word {
  int len;
  constexpr int twice() const { return 2 * len; }
};

constexpr bool test_split() {
  Counter c;
  int n = text::split(text::view("  the quick  brown fox "), ' ', c);
  return n == 4 && c.calls == 4 && c.longest == 5;
}

constexpr bool test_lambda() {
  size_t total = 0;
  text::split(text::view("a,bb,,ccc"), ',', [&](text::View w) { total += w.n; });
  auto upper = [](char ch) { return ch >= 'A' && ch <= 'Z'; };
  return total == 6 && text::count_if(text::view("HeLLo"), upper) == 3;
}

constexpr bool test_function_pointer() {
  return text::count_if(text::view("education"), is_vowel) == 5;
}

constexpr bool test_member_pointers() {
  Word w{4};
  auto pl = &Word::len;
  auto pt = &Word::twice;
  lib::FunctionRef<int(const Word&)> get = pl;
  lib::FunctionRef<int(const Word&)> dbl = pt;
  lib::FunctionRef<int(const Word*)> via_ptr = pt;
  return get(w) == 4 && dbl(w) == 8 && via_ptr(&w) == 8 && lib::invoke(pl, w) == 4;
}

constexpr int apply_twice(lib::FunctionRef<int(int)> f, int x) { return f(f(x)); }

// The thunk casts from void*, which constant evaluation does not allow in
// C++23, so the FunctionRef tests run at run time only.
static_assert(text::view("ab") == text::view("ab"));
static_assert(lib::invoke(&Word::twice, Word{3}) == 6);
static_assert(lib::invoke(&Word::len, Word{3}) == 3);
static_assert(lib::same<decltype(lib::invoke(&Word::len, lib::declval<Word&>())), int&>);
static_assert(lib::invocable_r<int (*)(int), long, short>);
static_assert(!lib::invocable_r<int (*)(int), text::View, int>);

int main() {
  int r = 0;
  if (!test_split()) r |= 1;
  if (!test_lambda()) r |= 2;
  if (!test_function_pointer()) r |= 4;
  if (!test_member_pointers()) r |= 8;
  int k = 5;
  if (apply_twice([&](int x) { return x + k; }, 1) != 11) r |= 16;
  if (apply_twice([](int x) { return x * 3; }, 2) != 18) r |= 32;
  return r;
}
