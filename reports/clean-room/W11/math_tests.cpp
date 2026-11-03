// Compile-time unit tests for an integer square root.
constexpr unsigned isqrt(unsigned n) {
  unsigned lo = 0, hi = n < 2 ? n : n / 2 + 1;
  while (lo < hi) {
    unsigned mid = lo + (hi - lo + 1) / 2;
    if (mid <= n / mid) lo = mid; else hi = mid - 1;
  }
  return lo;
}

struct Case { unsigned input; unsigned expected; };

constexpr Case cases[] = {
  {0, 0}, {1, 1}, {2, 1}, {3, 1}, {4, 2}, {15, 3}, {16, 4},
  {99, 9}, {100, 10}, {4294967295u, 65535u},
};

// Returns the index of the first failing case, or -1.
constexpr int first_failure() {
  int i = 0;
  for (const Case& c : cases) {
    if (isqrt(c.input) != c.expected) return i;
    ++i;
  }
  return -1;
}
static_assert(first_failure() == -1, "a table case failed");

// Property test over a range: r*r <= n < (r+1)*(r+1).
constexpr bool property_holds(unsigned from, unsigned to) {
  for (unsigned n = from; n < to; ++n) {
    unsigned long long r = isqrt(n);
    if (!(r * r <= n && n < (r + 1) * (r + 1))) return false;
  }
  return true;
}
static_assert(property_holds(0, 2000));
static_assert(property_holds(4294965000u, 4294967295u));

int main() {
  // The same function at run time.
  volatile unsigned n = 1000000;
  return isqrt(n) == 1000 && first_failure() == -1 ? 0 : 1;
}
