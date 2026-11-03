// W2: a clamp and a sort of three values. No headers.

// clamp with nested ?: (COND-6 groups to the right). All three operands
// are lvalues of type const int, so the result is an lvalue (COND-3) and
// the function can return a reference to the chosen argument.
constexpr const int& clamp(const int& v, const int& lo, const int& hi) {
  return v < lo ? lo : hi < v ? hi : v;
}

static_assert(clamp(5, 0, 9) == 5);
static_assert(clamp(-3, 0, 9) == 0);
static_assert(clamp(12, 0, 9) == 9);
static_assert(clamp(0, 0, 9) == 0 && clamp(9, 0, 9) == 9);

// The same for double. With a NaN both tests are false (CMP-5), so the
// NaN goes through unchanged.
constexpr double clamp(double v, double lo, double hi) {
  return v < lo ? lo : hi < v ? hi : v;
}
static_assert(clamp(2.5, 0.0, 1.0) == 1.0);
static_assert(clamp(-0.5, 0.0, 1.0) == 0.0);

// Mixed operands of ?: convert by the usual arithmetic conversions
// (COND-3): int and double give double.
constexpr auto pick(bool c) { return c ? 1 : 2.5; }
static_assert(pick(true) == 1.0);

// A conditional result that is a non-const lvalue can be assigned to.
// Only the selected operand is evaluated (COND-2).
constexpr int bump_smaller(int a, int b) {
  (a < b ? a : b) += 100;
  return a * 1000 + b;
}
static_assert(bump_smaller(1, 2) == 101 * 1000 + 2);
static_assert(bump_smaller(5, 2) == 5 * 1000 + 102);

// Sort three values in place with at most three swaps.
constexpr void swap_if_greater(int& x, int& y) {
  if (x > y) { int t = x; x = y; y = t; }
}
constexpr void sort3(int& a, int& b, int& c) {
  swap_if_greater(a, b);
  swap_if_greater(b, c);
  swap_if_greater(a, b);
}

struct Three { int a, b, c; };
constexpr Three sorted(int a, int b, int c) {
  sort3(a, b, c);
  return Three{a, b, c};
}
// "a <= b <= c" would compare a bool with c (CMP-1), so each pair is
// tested on its own.
constexpr bool in_order(Three t) { return t.a <= t.b && t.b <= t.c; }

static_assert(in_order(sorted(3, 2, 1)));
static_assert(in_order(sorted(1, 3, 2)));
static_assert(in_order(sorted(2, 1, 3)));
static_assert(in_order(sorted(2, 2, 1)));
static_assert(sorted(9, -4, 0).a == -4 && sorted(9, -4, 0).c == 9);

// Minimum, median, and maximum by ?: alone.
constexpr int min3(int a, int b, int c) { return a < b ? (a < c ? a : c) : (b < c ? b : c); }
constexpr int max3(int a, int b, int c) { return a > b ? (a > c ? a : c) : (b > c ? b : c); }
constexpr int mid3(int a, int b, int c) { return a + b + c - min3(a, b, c) - max3(a, b, c); }
static_assert(min3(4, 7, 1) == 1 && mid3(4, 7, 1) == 4 && max3(4, 7, 1) == 7);

// What a chained comparison really does, with parentheses to show it
// is meant (CMP-1, CMP-14): (3 > 2) is true, true becomes 1, 1 > 1 is
// false.
static_assert(((3 > 2) > 1) == false);

// The converted value is compared (CMP-3): -1 as unsigned is the largest
// unsigned value. The cast says the conversion is meant (CMP-4).
static_assert(!(static_cast<unsigned>(-1) < 1u));

int main() {
  // Check every order of three distinct values and some repeats.
  const int perms[6][3] = {{1,2,3},{1,3,2},{2,1,3},{2,3,1},{3,1,2},{3,2,1}};
  for (int i = 0; i < 6; ++i) {
    int a = perms[i][0], b = perms[i][1], c = perms[i][2];
    sort3(a, b, c);
    if (a != 1 || b != 2 || c != 3) return 1 + i;
  }

  // clamp returns a reference to one of its arguments.
  int v = 50, lo = 0, hi = 10;
  if (&clamp(v, lo, hi) != &hi) return 10;
  if (&clamp(lo, lo, hi) != &lo) return 11;

  // NaN passes through clamp unchanged.
  volatile double zero = 0.0;
  double nan = zero / zero;
  double r = clamp(nan, 0.0, 1.0);
  if (r == r) return 12;                 // still a NaN (CMP-5)
  return 0;
}
