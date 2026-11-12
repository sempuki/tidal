// Compile-time string keys: FixedString<N> as a constant template
// parameter made from a string literal, a key type per string, a literal
// operator template that makes keys, an FNV-1a hash, a sorted table of
// keys built in constant evaluation, and a record whose fields are looked
// up by key. No standard header. Workaround for tidal-fe: FixedString has a
// fixed capacity in place of a class template deduced from the literal.

using size_t = decltype(sizeof 0);
using u64 = unsigned long long;

namespace keys {

// A string of at most 15 characters. A class template FixedString<N>
// deduced from the literal is the usual form; this one has a fixed
// capacity.
struct FixedString {
  char text[16]{};
  size_t len = 0;

  constexpr FixedString(const char* s) {
    while (s[len] != 0 && len < 15) {
      text[len] = s[len];
      ++len;
    }
  }
  constexpr size_t size() const { return len; }
  constexpr char operator[](size_t i) const { return text[i]; }

  constexpr bool operator==(const FixedString& o) const {
    if (len != o.len) return false;
    for (size_t i = 0; i < len; ++i)
      if (text[i] != o.text[i]) return false;
    return true;
  }
};

constexpr u64 fnv1a(const char* s, size_t n) {
  u64 h = 14695981039346656037ull;
  for (size_t i = 0; i < n; ++i) {
    h ^= static_cast<unsigned char>(s[i]);
    h *= 1099511628211ull;
  }
  return h;
}

template <FixedString S>
struct Key {
  static constexpr auto name = S;
  static constexpr u64 hash = fnv1a(S.text, S.size());
};

template <class T> inline constexpr bool is_key = false;
template <FixedString S> inline constexpr bool is_key<Key<S>> = true;

template <class K>
concept KeyType = is_key<K>;

namespace literals {
template <FixedString S>
constexpr Key<S> operator""_k() { return {}; }
}  // namespace literals

// Compares two strings by their text, shorter first on a common prefix.
constexpr int compare(const char* a, size_t na, const char* b, size_t nb) {
  size_t n = na < nb ? na : nb;
  for (size_t i = 0; i < n; ++i)
    if (a[i] != b[i]) return a[i] < b[i] ? -1 : 1;
  return na < nb ? -1 : na > nb ? 1 : 0;
}

struct Entry {
  const char* text;
  size_t len;
  int value;
};

// A table of N entries, sorted by text when it is made.
template <size_t N>
struct Table {
  Entry e[N];

  constexpr Table(const Entry (&in)[N]) : e{} {
    for (size_t i = 0; i < N; ++i) e[i] = in[i];
    for (size_t i = 1; i < N; ++i)
      for (size_t j = i; j > 0 && compare(e[j].text, e[j].len, e[j - 1].text, e[j - 1].len) < 0; --j) {
        Entry t = e[j];
        e[j] = e[j - 1];
        e[j - 1] = t;
      }
  }

  constexpr int find(const char* s, size_t n, int missing = -1) const {
    size_t lo = 0, hi = N;
    while (lo < hi) {
      size_t mid = lo + (hi - lo) / 2;
      int c = compare(e[mid].text, e[mid].len, s, n);
      if (c == 0) return e[mid].value;
      if (c < 0) lo = mid + 1; else hi = mid;
    }
    return missing;
  }

  template <KeyType K>
  constexpr int operator[](K) const { return find(K::name.text, K::name.size()); }
};

template <size_t N>
Table(const Entry (&)[N]) -> Table<N>;

// A record of named int fields: Field<"x">, Field<"y">, ...
template <FixedString S>
struct Field {
  static constexpr auto name = S;
  int value = 0;
};

template <class... Fs>
struct Record : Fs... {
  template <FixedString S>
  constexpr int& get() { return static_cast<Field<S>&>(*this).value; }
  template <FixedString S>
  constexpr const int& get() const { return static_cast<const Field<S>&>(*this).value; }

  static constexpr size_t count = sizeof...(Fs);

  constexpr int sum() const { return (static_cast<const Fs&>(*this).value + ... + 0); }
};

}  // namespace keys

using namespace keys::literals;

constexpr keys::Entry raw[] = {
  {"pear", 4, 3}, {"apple", 5, 1}, {"fig", 3, 7}, {"banana", 6, 2}, {"apples", 6, 9},
};
constexpr keys::Table table(raw);

static_assert(table.e[0].value == 1 && table.e[1].value == 9);
static_assert(table.find("fig", 3) == 7);
static_assert(table.find("grape", 5) == -1);
static_assert(table["banana"_k] == 2);
static_assert(table[keys::Key<"pear">{}] == 3);

static_assert(keys::Key<"abc">::hash == keys::fnv1a("abc", 3));
static_assert(keys::Key<"abc">::hash != keys::Key<"abd">::hash);
static_assert(keys::is_key<decltype("x"_k)>);
static_assert(!keys::is_key<int>);
static_assert(keys::FixedString("ab") == keys::FixedString("ab"));
static_assert(!(keys::FixedString("ab") == keys::FixedString("abc")));

template <class A, class B>
inline constexpr bool same = false;
template <class A>
inline constexpr bool same<A, A> = true;

static_assert(same<decltype("id"_k), keys::Key<"id">>);
static_assert(!same<keys::Key<"id">, keys::Key<"ID">>);

using Point = keys::Record<keys::Field<"x">, keys::Field<"y">, keys::Field<"z">>;

constexpr int point_test() {
  Point p;
  p.get<"x">() = 3;
  p.get<"y">() = 4;
  p.get<"z">() += 5;
  const Point& c = p;
  return c.get<"x">() * 100 + c.get<"y">() * 10 + c.get<"z">() + c.sum() * 1000;
}
static_assert(Point::count == 3);
static_assert(point_test() == 12345);

int main() {
  int r = 0;
  if (table.find("apples", 6) != 9) r |= 1;
  if (table["fig"_k] != 7) r |= 2;
  if (point_test() != 12345) r |= 4;
  Point p;
  p.get<"y">() = 2;
  if (p.sum() != 2) r |= 8;
  return r;
}
