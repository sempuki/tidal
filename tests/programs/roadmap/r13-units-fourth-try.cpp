// tidal-rules: TSP-14 TARG-19 CNS-3 CNS-7 OVL-24 OVL-9 DED-31 INST-1 INST-2 LSUP-2
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-error-edg: no valid std::weak_ordering type found
// tidal-note: Q-0386. EDG checks every ordering class for the defaulted <=>, and the program declares only std::strong_ordering, the class it uses.
// tidal-note: Roadmap round 13, programmer sketch units_fourth_try.cpp (reports/clean-room/roadmap-13/programmer.md, section 1.4). The fourth try builds clean on GCC and Clang, so it is a program test (decision 0061). It differs from the last version in a constrained explicit conversion function in place of number(), decltype(10_m / 2_s)::dim, and decltype of nested operator calls as a template argument. The rules are the ones the report names for this program and its reductions.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Quantity<D, T>: a value with a physical dimension, where the dimension
// is a constant template parameter of class type. Dimensions add under *
// and subtract under /. Literals _m, _km, _s, _kg make quantities, a
// constrained conversion gives the number of a dimensionless quantity, a
// concept checks that a sum has one dimension, a trait matches speeds by
// partial specialization, and the unit text is made at compile time.
namespace std {
using size_t = decltype(sizeof 0);
struct strong_ordering {
  int v;
  static const strong_ordering less, equal, greater;
  friend constexpr bool operator==(strong_ordering a, strong_ordering b) = default;
  friend constexpr bool operator==(strong_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(strong_ordering a, int) { return a.v < 0; }
  friend constexpr bool operator>(strong_ordering a, int) { return a.v > 0; }
  friend constexpr bool operator<=(strong_ordering a, int) { return a.v <= 0; }
  friend constexpr bool operator>=(strong_ordering a, int) { return a.v >= 0; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}

template <class A, class B> inline constexpr bool same = false;
template <class A> inline constexpr bool same<A, A> = true;

struct Dim {
  int length = 0;
  int mass = 0;
  int time = 0;
  constexpr bool operator==(const Dim&) const = default;
  friend constexpr Dim operator+(Dim a, Dim b) {
    return {a.length + b.length, a.mass + b.mass, a.time + b.time};
  }
  friend constexpr Dim operator-(Dim a, Dim b) {
    return {a.length - b.length, a.mass - b.mass, a.time - b.time};
  }
};

inline constexpr Dim none{};
inline constexpr Dim length{1, 0, 0};
inline constexpr Dim mass{0, 1, 0};
inline constexpr Dim duration{0, 0, 1};

template <Dim D, class T = long long>
struct Quantity {
  using value_type = T;
  static constexpr Dim dim = D;
  T value{};

  constexpr Quantity() = default;
  constexpr explicit Quantity(T v) : value(v) {}

  constexpr explicit operator T() const requires (D == none) { return value; }

  constexpr Quantity& operator+=(Quantity o) { value += o.value; return *this; }
  constexpr Quantity& operator-=(Quantity o) { value -= o.value; return *this; }
  constexpr Quantity operator-() const { return Quantity(-value); }
  friend constexpr Quantity operator+(Quantity a, Quantity b) { return a += b; }
  friend constexpr Quantity operator-(Quantity a, Quantity b) { return a -= b; }
  friend constexpr Quantity operator*(T k, Quantity q) { return Quantity(k * q.value); }
  friend constexpr Quantity operator*(Quantity q, T k) { return Quantity(q.value * k); }
  constexpr auto operator<=>(const Quantity&) const = default;
};

template <Dim A, Dim B, class T>
constexpr Quantity<A + B, T> operator*(Quantity<A, T> a, Quantity<B, T> b) {
  return Quantity<A + B, T>(a.value * b.value);
}
template <Dim A, Dim B, class T>
constexpr Quantity<A - B, T> operator/(Quantity<A, T> a, Quantity<B, T> b) {
  return Quantity<A - B, T>(a.value / b.value);
}

using Meters = Quantity<length>;
using Seconds = Quantity<duration>;
using Kilograms = Quantity<mass>;
using Speed = Quantity<length - duration>;
using Accel = Quantity<length - duration - duration>;
using Force = Quantity<mass + length - duration - duration>;

namespace literals {
constexpr Meters operator""_m(unsigned long long v) { return Meters(static_cast<long long>(v)); }
constexpr Meters operator""_km(unsigned long long v) { return Meters(static_cast<long long>(v) * 1000); }
constexpr Seconds operator""_s(unsigned long long v) { return Seconds(static_cast<long long>(v)); }
constexpr Kilograms operator""_kg(unsigned long long v) { return Kilograms(static_cast<long long>(v)); }
}
using namespace literals;

template <class Q> concept AnyQuantity = requires {
  typename Q::value_type;
  Q::dim;
};
template <class A, class B> concept SameDim = AnyQuantity<A> && AnyQuantity<B> && (A::dim == B::dim);

template <class Q, class... Qs> inline constexpr bool all_same_dim = (SameDim<Q, Qs> && ...);

template <AnyQuantity Q, AnyQuantity... Qs>
  requires all_same_dim<Q, Qs...>
constexpr Q total(Q first, Qs... rest) {
  return (first + ... + rest);
}

template <AnyQuantity Q> constexpr Q magnitude(Q q) { return q.value < 0 ? -q : q; }

template <class Q> struct is_speed { static constexpr bool value = Q::dim == length - duration; };

// The unit text: "m kg s^-2" and so on, made in constant evaluation.
struct UnitText {
  char c[32]{};
  int n = 0;
  constexpr void put(char ch) { c[n++] = ch; }
  constexpr void term(const char* name, int power) {
    if (power == 0) return;
    if (n) put(' ');
    for (; *name; ++name) put(*name);
    if (power != 1) {
      put('^');
      if (power < 0) { put('-'); power = -power; }
      put(static_cast<char>('0' + power));
    }
  }
  constexpr bool equals(const char* s) const {
    int i = 0;
    for (; s[i]; ++i) if (i >= n || c[i] != s[i]) return false;
    return i == n;
  }
};
template <Dim D> constexpr UnitText unit() {
  UnitText t;
  t.term("m", D.length);
  t.term("kg", D.mass);
  t.term("s", D.time);
  return t;
}

template <AnyQuantity Q> struct Vec2 {
  Q x, y;
  friend constexpr Vec2 operator+(Vec2 a, Vec2 b) { return {a.x + b.x, a.y + b.y}; }
  constexpr bool operator==(const Vec2&) const = default;
};
template <class Q> Vec2(Q, Q) -> Vec2<Q>;

// Constant tests.
static_assert((10_m / 2_s).value == 5);
static_assert(decltype(10_m / 2_s)::dim == length - duration);
static_assert(same<decltype(10_m / 2_s), Speed>);
static_assert(same<decltype(2_kg * (3_m / (1_s * 1_s))), Force>);
static_assert(static_cast<long long>(6_m / 3_m) == 2);
static_assert(3_km == 3000_m);
static_assert(2_m < 3_m && 3_s > 2_s);
static_assert(total(1_m, 2_m, 3_km).value == 3003);
static_assert(magnitude(-(5_s)) == 5_s);
static_assert(is_speed<Speed>::value && !is_speed<Accel>::value);
static_assert(unit<Force::dim>().equals("m kg s^-2"));
static_assert(unit<none>().equals(""));
static_assert((Vec2{1_m, 2_m} + Vec2{3_m, 4_m}) == Vec2{4_m, 6_m});

template <class A, class B> concept Addable = requires(A a, B b) { a + b; };
static_assert(Addable<Meters, Meters> && !Addable<Meters, Seconds>);
template <class Q> concept HasNumber = requires(Q q) { static_cast<long long>(q); };
static_assert(HasNumber<Quantity<none>> && !HasNumber<Meters>);

// A run: an object falls for 10 steps of 1 s under 10 m/s^2, with
// speed and height in whole units.
int main() {
  constexpr Accel g = 10_m / (1_s * 1_s);
  Meters height = 1_km;
  Speed v{};
  Seconds t{};
  const Seconds dt = 1_s;
  int steps = 0;
  while (height > 0_m && steps < 100) {
    v += g * dt;
    height -= v * dt;
    t += dt;
    ++steps;
  }
  if (steps != 14) return 1;
  if (t != 14_s) return 2;
  if (v.value != 140) return 3;
  Force f = 2_kg * g;
  if (f.value != 20) return 4;
  auto text = unit<decltype(f)::dim>();
  if (!text.equals("m kg s^-2")) return 5;
  Vec2 p{3_m, 4_m};
  Vec2 q = p + p;
  if (q.x.value != 6 || q.y.value != 8) return 6;
  return 0;
}
