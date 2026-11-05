// inventory.cpp: a small inventory ledger with no headers.
// Round 6 sketch, programmer view. Shapes from the language-support page.
namespace std {
using size_t = decltype(sizeof 0);

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

struct partial_ordering {
  signed char v;
  static const partial_ordering less, equivalent, greater, unordered;
};
inline constexpr partial_ordering partial_ordering::less{-1};
inline constexpr partial_ordering partial_ordering::equivalent{0};
inline constexpr partial_ordering partial_ordering::greater{1};
inline constexpr partial_ordering partial_ordering::unordered{2};
struct weak_ordering {
  signed char v;
  static const weak_ordering less, equivalent, greater;
};
inline constexpr weak_ordering weak_ordering::less{-1};
inline constexpr weak_ordering weak_ordering::equivalent{0};
inline constexpr weak_ordering weak_ordering::greater{1};
struct strong_ordering {
  signed char v;
  static const strong_ordering less, equal, equivalent, greater;
  friend constexpr bool operator==(strong_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator!=(strong_ordering a, int) { return a.v != 0; }
  friend constexpr bool operator<(strong_ordering a, int) { return a.v < 0; }
  friend constexpr bool operator>(strong_ordering a, int) { return a.v > 0; }
  friend constexpr bool operator<=(strong_ordering a, int) { return a.v <= 0; }
  friend constexpr bool operator>=(strong_ordering a, int) { return a.v >= 0; }
  friend constexpr bool operator<(int, strong_ordering a) { return 0 < a.v; }
  friend constexpr bool operator>(int, strong_ordering a) { return 0 > a.v; }
  friend constexpr bool operator==(strong_ordering, strong_ordering) = default;
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};

class exception {
public:
  virtual ~exception();
};
class bad_cast : public exception {
public:
  ~bad_cast() override;
};
}  // namespace std

// Money in cents, with a literal suffix.
struct Cents {
  long long value;
  auto operator<=>(const Cents&) const = default;
  Cents operator+(Cents o) const { return {value + o.value}; }
  Cents operator*(int n) const { return {value * n}; }
};
constexpr Cents operator""_c(unsigned long long v) { return {static_cast<long long>(v)}; }

struct Sku {
  int department;
  int number;
  auto operator<=>(const Sku&) const = default;
};

struct Item {
  Sku sku;
  Cents price;
  int count;
};

// A growable array that owns its elements.
template <class T> class Vec {
  T* data = nullptr;
  std::size_t used = 0;
  std::size_t cap = 0;

  void grow() {
    std::size_t next = cap == 0 ? 4 : cap * 2;
    T* fresh = new T[next];
    for (std::size_t i = 0; i < used; ++i) fresh[i] = static_cast<T&&>(data[i]);
    delete[] data;
    data = fresh;
    cap = next;
  }

public:
  Vec() = default;
  Vec(std::initializer_list<T> list) {
    for (const T& x : list) push(x);
  }
  Vec(const Vec& o) {
    for (const T& x : o) push(x);
  }
  Vec(Vec&& o) noexcept : data(o.data), used(o.used), cap(o.cap) {
    o.data = nullptr;
    o.used = o.cap = 0;
  }
  Vec& operator=(Vec o) noexcept {
    T* d = data; data = o.data; o.data = d;
    std::size_t u = used; used = o.used; o.used = u;
    std::size_t c = cap; cap = o.cap; o.cap = c;
    return *this;
  }
  ~Vec() { delete[] data; }

  void push(const T& x) {
    if (used == cap) grow();
    data[used++] = x;
  }
  // Keep the array sorted by key: insert before the first larger element.
  template <class Key, class U = T> void insert_sorted(const T& x, Key U::* key) {
    push(x);
    std::size_t i = used - 1;
    while (i > 0 && x.*key < data[i - 1].*key) {
      data[i] = data[i - 1];
      --i;
    }
    data[i] = x;
  }
  T* begin() { return data; }
  T* end() { return data + used; }
  const T* begin() const { return data; }
  const T* end() const { return data + used; }
  std::size_t size() const { return used; }
  T& operator[](std::size_t i) { return data[i]; }
};

template <class... Ts> constexpr auto sum(Ts... xs) { return (xs + ...); }

template <class Range, class Pred> int count_if(const Range& r, Pred p) {
  int n = 0;
  for (const auto& x : r)
    if (p(x)) ++n;
  return n;
}

template <class T> constexpr int weight(const T& x) {
  if constexpr (requires { x.count; }) return x.count;
  else return 1;
}

consteval int squares_up_to(int n) {
  int* table = new int[n];
  for (int i = 0; i < n; ++i) table[i] = i * i;
  int total = 0;
  for (int i = 0; i < n; ++i) total += table[i];
  delete[] table;
  return total;
}
static_assert(squares_up_to(4) == 14);

struct Shelf { virtual ~Shelf() = default; virtual int slots() const { return 0; } };
struct Rack : Shelf { int slots() const override { return 12; } };
struct Bin : Shelf { int slots() const override { return 1; } };

int rack_slots(const Shelf& s) {
  try {
    return dynamic_cast<const Rack&>(s).slots();
  } catch (const std::exception&) {
    return -1;
  }
}

int main() {
  int bad = 0;
  Vec<Item> stock;
  stock.insert_sorted({{2, 7}, 150_c, 3}, &Item::sku);
  stock.insert_sorted({{1, 9}, 99_c, 10}, &Item::sku);
  stock.insert_sorted({{2, 1}, 1200_c, 1}, &Item::sku);
  stock.insert_sorted({{1, 2}, 5_c, 40}, &Item::sku);
  stock.insert_sorted({{3, 0}, 250_c, 0}, &Item::sku);

  Sku expect[] = {{1, 2}, {1, 9}, {2, 1}, {2, 7}, {3, 0}};
  for (std::size_t i = 0; i < stock.size(); ++i)
    if (stock[i].sku != expect[i]) ++bad;

  Cents total = 0_c;
  for (const auto& [sku, price, count] : stock) total = total + price * count;
  if (total != 200_c + 990_c + 1200_c + 450_c) ++bad;

  int out = count_if(stock, [](const Item& x) { return x.count == 0; });
  if (out != 1) ++bad;

  int limit = 5;
  int few = count_if(stock, [&limit](const Item& x) { return x.count < limit; });
  if (few != 3) ++bad;

  Vec<int> ns = {4, 5, 6};
  Vec<int> copy = ns;
  ns = Vec<int>{1};
  if (ns.size() != 1 || copy.size() != 3 || copy[2] != 6) ++bad;

  if (sum(1, 2, 3, 4) != 10) ++bad;
  if (weight(stock[0]) != 40 || weight(7) != 1) ++bad;
  if (!(Sku{1, 2} < Sku{1, 3}) || (Cents{5} <=> Cents{4}) <= 0) ++bad;

  Rack r;
  Bin b;
  if (rack_slots(r) != 12 || rack_slots(b) != -1) ++bad;
  return bad;
}
