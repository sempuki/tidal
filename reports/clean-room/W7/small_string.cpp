// W7: a small string with fixed capacity, written from the Tidal
// specification. The characters live inside the object; nothing is
// allocated.
namespace std {
using size_t = decltype(sizeof 0);                 // LSUP-4

// LSUP-1: the shape the language needs. The example on the
// language-support page gives this form.
template <class E> class initializer_list {
  const E* first;
  size_t count;
  constexpr initializer_list(const E* f, size_t n) : first(f), count(n) {}
public:
  constexpr initializer_list() : first(nullptr), count(0) {}
  constexpr const E* begin() const { return first; }
  constexpr const E* end() const { return first + count; }
  constexpr size_t size() const { return count; }
};

struct strong_ordering {                           // LSUP-2
  signed char value;
  static const strong_ordering less;
  static const strong_ordering equal;
  static const strong_ordering equivalent;
  static const strong_ordering greater;
  friend constexpr bool operator==(strong_ordering a, strong_ordering b) { return a.value == b.value; }
  friend constexpr bool operator==(strong_ordering a, int) { return a.value == 0; }
  friend constexpr bool operator<(strong_ordering a, int) { return a.value < 0; }
  friend constexpr bool operator>(strong_ordering a, int) { return a.value > 0; }
  friend constexpr bool operator<=(strong_ordering a, int) { return a.value <= 0; }
  friend constexpr bool operator>=(strong_ordering a, int) { return a.value >= 0; }
  friend constexpr bool operator<(int, strong_ordering a) { return 0 < a.value; }
  friend constexpr bool operator>(int, strong_ordering a) { return 0 > a.value; }
  friend constexpr bool operator<=(int, strong_ordering a) { return 0 <= a.value; }
  friend constexpr bool operator>=(int, strong_ordering a) { return 0 >= a.value; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}

struct Full {};
struct OutOfRange {};

template <std::size_t N>
class SmallString {
  char data_[N + 1] = {};      // CTOR-5: default member initializer, all zero
  std::size_t size_ = 0;

public:
  // CTOR-2: other constructors remove the implicit default; = default
  // brings it back. ILC-2: {} calls this one.
  constexpr SmallString() = default;

  // A string literal of M characters, the last one '\0'.
  // CPT-14: a trailing requires clause needs a templated function.
  template <std::size_t M>
    requires(M - 1 <= N)
  constexpr SmallString(const char (&text)[M]) {
    for (std::size_t i = 0; i + 1 < M; ++i) data_[i] = text[i];
    size_ = M - 1;
  }

  // ILC-3: a braced list of chars picks this constructor first.
  constexpr SmallString(std::initializer_list<char> chars) {
    for (char c : chars) push_back(c);
  }

  static constexpr std::size_t capacity() { return N; }
  constexpr std::size_t size() const { return size_; }
  constexpr bool empty() const { return size_ == 0; }
  constexpr const char* c_str() const { return data_; }

  constexpr void push_back(char c) {
    if (size_ == N) throw Full{};
    data_[size_++] = c;
    data_[size_] = '\0';
  }
  constexpr SmallString& operator+=(char c) { push_back(c); return *this; }

  template <std::size_t M>
  constexpr SmallString& operator+=(const SmallString<M>& other) {
    for (char c : other) push_back(c);
    return *this;
  }

  // CLS-5, CLS-25: a const object can only call the const overload.
  constexpr char& operator[](std::size_t i) {
    if (i >= size_) throw OutOfRange{};
    return data_[i];
  }
  constexpr const char& operator[](std::size_t i) const {
    if (i >= size_) throw OutOfRange{};
    return data_[i];
  }

  // RFOR: begin and end members make a range.
  constexpr char* begin() { return data_; }
  constexpr char* end() { return data_ + size_; }
  constexpr const char* begin() const { return data_; }
  constexpr const char* end() const { return data_ + size_; }

  // Equality across capacities. DCMP-6: b == a is tried reversed too.
  template <std::size_t M>
  friend constexpr bool operator==(const SmallString& a, const SmallString<M>& b) {
    if (a.size() != b.size()) return false;
    for (std::size_t i = 0; i < a.size(); ++i)
      if (a[i] != b[i]) return false;
    return true;
  }
  friend constexpr bool operator==(const SmallString& a, const char* s) {
    std::size_t i = 0;
    for (; i < a.size_; ++i)
      if (s[i] != a.data_[i]) return false;
    return s[i] == '\0';
  }

  // Ordering within one capacity, character by character.
  friend constexpr std::strong_ordering operator<=>(const SmallString& a, const SmallString& b) {
    std::size_t n = a.size_ < b.size_ ? a.size_ : b.size_;
    for (std::size_t i = 0; i < n; ++i)
      if (a.data_[i] != b.data_[i])
        return static_cast<unsigned char>(a.data_[i]) <=> static_cast<unsigned char>(b.data_[i]);
    return a.size_ <=> b.size_;
  }
};

// CTAD-11: a user guide lets SmallString s = "abc"; deduce the capacity.
template <std::size_t M>
SmallString(const char (&)[M]) -> SmallString<M - 1>;

template <std::size_t N, std::size_t M>
constexpr SmallString<N + M> operator+(const SmallString<N>& a, const SmallString<M>& b) {
  SmallString<N + M> r;
  r += a;
  r += b;
  return r;
}

constexpr SmallString hello = "hello";             // CTAD-11
static_assert(hello.capacity() == 5);
static_assert(hello.size() == 5);
static_assert(hello == "hello");
static_assert("hello" == hello);                   // reversed (DCMP-6)
static_assert(hello != "help");                    // rewritten (DCMP-6)
static_assert(hello[1] == 'e');

constexpr SmallString<8> word = "hi";
static_assert(word.capacity() == 8 && word.size() == 2);
static_assert(word == SmallString<2>("hi"));       // across capacities
static_assert(SmallString<2>("hi") == word);

constexpr SmallString<4> braced{'a', 'b', 'c'};    // ILC-3
static_assert(braced == "abc");
constexpr SmallString<4> none{};                   // ILC-2
static_assert(none.empty());

static_assert(SmallString<4>("abc") < SmallString<4>("abd"));
static_assert(SmallString<4>("ab") < SmallString<4>("abc"));
static_assert(SmallString<4>("b") > SmallString<4>("abc"));

constexpr auto joined = hello + SmallString(" you");
static_assert(joined.capacity() == 9);
static_assert(joined == "hello you");

constexpr SmallString<6> built() {
  SmallString<6> s;
  s += 'a';
  s += 'b';
  s[0] = 'z';
  return s;
}
static_assert(built() == "zb");

// CPT-24: test what the type refuses.
template <std::size_t N, std::size_t M> concept Fits = requires(const char (&t)[M]) { SmallString<N>(t); };
static_assert(Fits<3, 4>);                         // "abc" fits in 3
static_assert(!Fits<3, 5>);                        // "abcd" does not
template <class S> concept CharWritable = requires(S s) { s[0] = 'x'; };
static_assert(CharWritable<SmallString<3>&>);
static_assert(!CharWritable<const SmallString<3>&>);   // CLS-25

int main() {
  SmallString<3> s;
  s += 'x';
  s += 'y';
  s += 'z';
  bool full = false;
  try {
    s += 'w';
  } catch (Full) {
    full = true;
  }
  if (!full) return 1;
  if (!(s == "xyz")) return 2;

  bool range = false;
  try {
    (void)s[3];
  } catch (OutOfRange) {
    range = true;
  }
  if (!range) return 3;

  int count = 0;
  for (char& c : s) { c = static_cast<char>(c - 'a' + 'A'); ++count; }
  if (count != 3 || s != "XYZ") return 4;

  SmallString<16> greeting = "hi";
  greeting += SmallString<5>(", bob");
  if (greeting != "hi, bob" || greeting.c_str()[7] != '\0') return 5;
  return 0;
}
