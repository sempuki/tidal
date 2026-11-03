// W2: a three-way compare for a record type.
// No headers. The ordering classes follow LSUP-2, with the comparisons
// against a literal 0 that the rewrites of OVL-13 call.
namespace std {
// The three ordering classes of LSUP-2. Each holds -1, 0, 1, or (for
// partial_ordering) 2 for unordered. The comparisons with a literal 0
// are what the rewritten candidates of OVL-13 call.
struct partial_ordering {
  int v;
  static const partial_ordering less, equivalent, greater, unordered;
  friend constexpr bool operator==(partial_ordering, partial_ordering) = default;
  friend constexpr bool operator==(partial_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(partial_ordering a, int) { return a.v == -1; }
  friend constexpr bool operator>(partial_ordering a, int) { return a.v == 1; }
  friend constexpr bool operator<=(partial_ordering a, int) { return a.v == -1 || a.v == 0; }
  friend constexpr bool operator>=(partial_ordering a, int) { return a.v == 1 || a.v == 0; }
  friend constexpr bool operator<(int, partial_ordering a) { return a.v == 1; }
  friend constexpr bool operator>(int, partial_ordering a) { return a.v == -1; }
  friend constexpr bool operator<=(int, partial_ordering a) { return a.v == 1 || a.v == 0; }
  friend constexpr bool operator>=(int, partial_ordering a) { return a.v == -1 || a.v == 0; }
};
inline constexpr partial_ordering partial_ordering::less{-1};
inline constexpr partial_ordering partial_ordering::equivalent{0};
inline constexpr partial_ordering partial_ordering::greater{1};
inline constexpr partial_ordering partial_ordering::unordered{2};

struct weak_ordering {
  int v;
  static const weak_ordering less, equivalent, greater;
  constexpr operator partial_ordering() const { return partial_ordering{v}; }
  friend constexpr bool operator==(weak_ordering, weak_ordering) = default;
  friend constexpr bool operator==(weak_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(weak_ordering a, int) { return a.v < 0; }
  friend constexpr bool operator>(weak_ordering a, int) { return a.v > 0; }
  friend constexpr bool operator<=(weak_ordering a, int) { return a.v <= 0; }
  friend constexpr bool operator>=(weak_ordering a, int) { return a.v >= 0; }
  friend constexpr bool operator<(int, weak_ordering a) { return 0 < a.v; }
  friend constexpr bool operator>(int, weak_ordering a) { return 0 > a.v; }
  friend constexpr bool operator<=(int, weak_ordering a) { return 0 <= a.v; }
  friend constexpr bool operator>=(int, weak_ordering a) { return 0 >= a.v; }
};
inline constexpr weak_ordering weak_ordering::less{-1};
inline constexpr weak_ordering weak_ordering::equivalent{0};
inline constexpr weak_ordering weak_ordering::greater{1};

struct strong_ordering {
  int v;
  static const strong_ordering less, equal, equivalent, greater;
  constexpr operator partial_ordering() const { return partial_ordering{v}; }
  constexpr operator weak_ordering() const { return weak_ordering{v}; }
  friend constexpr bool operator==(strong_ordering, strong_ordering) = default;
  friend constexpr bool operator==(strong_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(strong_ordering a, int) { return a.v < 0; }
  friend constexpr bool operator>(strong_ordering a, int) { return a.v > 0; }
  friend constexpr bool operator<=(strong_ordering a, int) { return a.v <= 0; }
  friend constexpr bool operator>=(strong_ordering a, int) { return a.v >= 0; }
  friend constexpr bool operator<(int, strong_ordering a) { return 0 < a.v; }
  friend constexpr bool operator>(int, strong_ordering a) { return 0 > a.v; }
  friend constexpr bool operator<=(int, strong_ordering a) { return 0 <= a.v; }
  friend constexpr bool operator>=(int, strong_ordering a) { return 0 >= a.v; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}

// Defaulted <=> (DCMP-4): members in declaration order, first difference
// wins. With auto the result is the weakest member category, here strong.
// The defaulted <=> also declares a defaulted == (DCMP-4).
struct Date {
  int year, month, day;
  auto operator<=>(const Date&) const = default;
};

constexpr Date d1{2026, 3, 9}, d2{2026, 11, 3}, d3{2026, 11, 3};
static_assert(d1 < d2);                 // (d1 <=> d2) < 0, OVL-13
static_assert(d2 > d1);
static_assert(d1 <= d2 && d2 <= d3);
static_assert(d2 >= d3);
static_assert(d2 == d3);                // the implied defaulted ==
static_assert(d1 != d2);                // !(d1 == d2), DCMP-6
static_assert((d1 <=> d2) == std::strong_ordering::less);
static_assert((d2 <=> d3) == std::strong_ordering::equal);
static_assert((Date{2027, 1, 1} <=> d2) == std::strong_ordering::greater);

// A hand-written <=>: department ascending, then salary descending,
// then id ascending. A <=> that is not defaulted brings no ==
// (DCMP-4, DCMP-10), so == is defaulted on its own.
struct Employee {
  int dept;
  long salary;
  int id;

  constexpr std::strong_ordering operator<=>(const Employee& o) const {
    if (auto c = dept <=> o.dept; c != 0) return c;
    if (auto c = o.salary <=> salary; c != 0) return c;  // reversed: descending
    return id <=> o.id;
  }
  bool operator==(const Employee&) const = default;
};

constexpr Employee alice{1, 9000, 7}, bob{1, 12000, 3}, carol{2, 1000, 1};
static_assert(bob < alice);             // same dept, higher salary first
static_assert(alice < carol);           // lower dept first
static_assert(!(alice < alice) && alice <= alice);
static_assert(alice == Employee{1, 9000, 7});
static_assert(alice != bob);
static_assert((carol <=> bob) == std::strong_ordering::greater);

// A double member makes the defaulted result partial_ordering (DCMP-4,
// CMP-17). The int member's strong_ordering converts to it.
struct Reading {
  int sensor;
  double value;
  auto operator<=>(const Reading&) const = default;
};

constexpr Reading r1{4, 1.5}, r2{4, 2.5};
static_assert(r1 < r2);
static_assert((r1 <=> r1) == std::partial_ordering::equivalent);
static_assert((Reading{3, 9.0} <=> r1) == std::partial_ordering::less);
static_assert((-0.0 <=> 0.0) == std::partial_ordering::equivalent);  // CMP-17

// A NaN can only be made at run time (MUL-11; constant evaluation
// rejects 0.0 / 0.0).
int check_nan() {
  volatile double zero = 0.0;
  double nan = zero / zero;
  Reading a{4, nan};
  if ((a <=> a) != std::partial_ordering::unordered) return 1;   // CMP-17
  if (a < a || a > a || a <= a || a >= a) return 2;              // CMP-5
  if (a == a) return 3;                  // defaulted == compares the NaN
  if (!(a != a)) return 4;
  if ((Reading{3, nan} <=> a) != std::partial_ordering::less) return 5;  // sensor decides first
  return 0;
}

// Sort records with the record's own <, at run time.
int check_sort() {
  Employee staff[4] = {carol, alice, Employee{1, 9000, 2}, bob};
  for (int i = 1; i < 4; ++i)
    for (int j = i; j > 0 && staff[j] < staff[j - 1]; --j) {
      Employee t = staff[j];
      staff[j] = staff[j - 1];
      staff[j - 1] = t;
    }
  const int want[4] = {3, 2, 7, 1};
  for (int i = 0; i < 4; ++i)
    if (staff[i].id != want[i]) return 10 + i;
  return 0;
}

int main() {
  if (int e = check_nan()) return e;
  return check_sort();
}
