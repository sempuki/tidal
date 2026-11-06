// Result<T, E>: a tag and an anonymous union of the two alternatives.
// Rules used: UNI-3 and UNI-11 (the class writes its special members),
// UNI-4 and UNI-7 (mem_init picks the active member), UNI-14 and LSUP-10
// (std::construct_at makes a member active, also in constant evaluation),
// DTOR-6 (explicit destructor call ends the active member), CPT-8 and
// CPT-31 (requires clause with a requires expression), DTY-2 (decltype of
// the call gives the result type), RET-1 and RET-2 (auto return type,
// both returns give Result<U, E>), CE-P1 and CE-P2.

namespace std {
using size_t = decltype(sizeof 0);
}
inline void* operator new(std::size_t, void* at) noexcept { return at; }
namespace std {
template <class T, class... Args>
constexpr T* construct_at(T* location, Args&&... args) {
  return ::new (static_cast<void*>(location)) T(static_cast<Args&&>(args)...);
}
}

struct OkTag {};
struct ErrTag {};

template <class T, class E>
class Result {
  bool ok_;
  union { T value_; E error_; };
public:
  constexpr Result(OkTag, const T& v) : ok_(true), value_(v) {}
  constexpr Result(ErrTag, const E& e) : ok_(false), error_(e) {}
  static constexpr Result ok(const T& v) { return Result(OkTag{}, v); }
  static constexpr Result err(const E& e) { return Result(ErrTag{}, e); }

  constexpr Result(const Result& o) : ok_(o.ok_) {
    if (ok_) std::construct_at(&value_, o.value_);
    else std::construct_at(&error_, o.error_);
  }
  constexpr Result& operator=(const Result& o) {
    if (this != &o) {
      destroy();
      ok_ = o.ok_;
      if (ok_) std::construct_at(&value_, o.value_);
      else std::construct_at(&error_, o.error_);
    }
    return *this;
  }
  constexpr ~Result() { destroy(); }

  constexpr bool is_ok() const { return ok_; }
  constexpr const T& value() const { return value_; }
  constexpr const E& error() const { return error_; }
  constexpr T value_or(const T& fallback) const { return ok_ ? value_ : fallback; }

  template <class F>
    requires requires(F f, const T& t) { f(t); }
  constexpr auto transform(F f) const {
    using U = decltype(f(value_));
    if (ok_) return Result<U, E>::ok(f(value_));
    return Result<U, E>::err(error_);
  }
private:
  constexpr void destroy() {
    if (ok_) value_.~T();
    else error_.~E();
  }
};

struct Error { int code; };

constexpr Result<int, Error> parse_digit(char c) {
  if (c >= '0' && c <= '9') return Result<int, Error>::ok(c - '0');
  return Result<int, Error>::err(Error{c});
}

static_assert(parse_digit('7').value_or(-1) == 7);
static_assert(parse_digit('x').value_or(-1) == -1);
static_assert(parse_digit('x').error().code == 'x');
static_assert(parse_digit('4').transform([](int v) { return v * 2.5; }).value() == 10.0);
static_assert(!parse_digit('q').transform([](int v) { return v > 3; }).is_ok());
constexpr bool copy_check() {
  Result<int, Error> a = parse_digit('3');
  Result<int, Error> b = parse_digit('z');
  b = a;
  return b.is_ok() && b.value() == 3;
}
static_assert(copy_check());

// A run-time check with an alternative that counts its lifetimes.
struct Text {
  static int live;
  const char* s;
  Text(const char* p) : s(p) { ++live; }
  Text(const Text& o) : s(o.s) { ++live; }
  Text& operator=(const Text& o) { s = o.s; return *this; }
  ~Text() { --live; }
};
int Text::live = 0;

int run(int argc) {
  {
    Result<Text, Error> r = argc > 0 ? Result<Text, Error>::ok(Text("abc"))
                                     : Result<Text, Error>::err(Error{1});
    if (!r.is_ok() || r.value().s[0] != 'a') return 1;
    auto len = r.transform([](const Text& t) { int n = 0; while (t.s[n]) ++n; return n; });
    if (len.value_or(0) != 3) return 2;
    Result<Text, Error> e = Result<Text, Error>::err(Error{9});
    r = e;
    if (r.is_ok() || r.error().code != 9) return 3;
    if (r.value_or(Text("fallback")).s[0] != 'f') return 4;
    auto bad = r.transform([](const Text&) { return 1; });
    if (bad.is_ok() || bad.error().code != 9) return 5;
  }
  if (Text::live != 0) return 6;
  return 0;
}

int main(int argc, char**) { return run(argc); }
