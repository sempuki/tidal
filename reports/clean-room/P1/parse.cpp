// Port of rust/parse.rs: Result, the ? operator, and an error enum.
//   enum ParseError { Empty, BadDigit(char), Overflow }
//        -> a struct with a scoped enumeration tag and a payload member,
//           after the Token example of unions.md; defaulted == (DCMP-1)
//   Result<T, E> -> a class template with a tag and an anonymous union
//           (unions.md); == written by hand, because DCMP-2 deletes a
//           defaulted == for a class with an anonymous union member
//   Err(e)       -> Err{e}, an aggregate deduced by CTAD-9
//   ?            -> no C++ form; an explicit test and early return
//   Result == Err(e) -> a second hidden friend operator== (DCMP-P2 gives
//           the rewrite of != and of the swapped operands)

using u8 = unsigned char;
using u16 = unsigned short;
using u32 = unsigned int;

struct ParseError {
  enum class Kind : unsigned char { empty, bad_digit, overflow };
  Kind kind;
  char digit = '\0';   // the payload of bad_digit
  static constexpr ParseError Empty() { return {Kind::empty}; }
  static constexpr ParseError BadDigit(char c) { return {Kind::bad_digit, c}; }
  static constexpr ParseError Overflow() { return {Kind::overflow}; }
  friend constexpr bool operator==(const ParseError&, const ParseError&) = default;
};

template <class E> struct Err { E error; };

template <class T, class E> class Result {
  bool ok_;
  union {
    T value_;
    E error_;
  };
public:
  constexpr Result(T v) : ok_(true), value_(v) {}
  constexpr Result(Err<E> e) : ok_(false), error_(e.error) {}
  constexpr bool is_ok() const { return ok_; }
  constexpr T value() const { return value_; }
  constexpr E error() const { return error_; }
  friend constexpr bool operator==(const Result& a, const Result& b) {
    if (a.ok_ != b.ok_) return false;
    return a.ok_ ? a.value_ == b.value_ : a.error_ == b.error_;
  }
  // Result == Err{e} without a conversion of Err to Result. The front
  // end rejects that conversion to const Result& (finding P1-2).
  friend constexpr bool operator==(const Result& a, const Err<E>& b) {
    return !a.ok_ && a.error_ == b.error;
  }
};

constexpr Result<u32, ParseError> to_digit(char c) {
  if (c < '0' || c > '9') return Err{ParseError::BadDigit(c)};
  return u32(c - '0');
}

constexpr Result<u8, ParseError> parse_u8(const char* s) {
  if (*s == '\0') return Err{ParseError::Empty()};
  u32 v = 0;
  for (; *s != '\0'; ++s) {
    auto d = to_digit(*s);
    if (!d.is_ok()) return Err{d.error()};          // ?
    v = v * 10 + d.value();
    if (v > 255) return Err{ParseError::Overflow()};
  }
  return u8(v);
}

constexpr Result<u16, ParseError> sum_pair(const char* a, const char* b) {
  auto x = parse_u8(a);
  if (!x.is_ok()) return Err{x.error()};            // ?
  auto y = parse_u8(b);
  if (!y.is_ok()) return Err{y.error()};            // ?
  return u16(x.value() + y.value());
}

constexpr bool run() {
  return sum_pair("200", "55") == u16(255)
      && parse_u8("") == Err{ParseError::Empty()}
      && parse_u8("4x") == Err{ParseError::BadDigit('x')}
      && parse_u8("256") == Err{ParseError::Overflow()};
}

static_assert(run());

int main() { return run() ? 0 : 1; }
