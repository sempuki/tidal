// tidal-rules: RET-1 RET-3 RET-7 RET-4 DEFARG-1 DEFARG-3 DTY-2 CE-10
// tidal-expect: run
// tidal-note: Write suite W5, program recursion.cpp (reports/clean-room/W5/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Recursion with a deduced return type.

template <class A, class B> constexpr bool same_type = false;
template <class A> constexpr bool same_type<A, A> = true;

constexpr auto fact(int n) {
  if (n <= 1) return 1;          // the type is now int
  return n * fact(n - 1);
}

constexpr auto fib(int n) {
  if (n < 2) return static_cast<long long>(n);
  return fib(n - 1) + fib(n - 2);
}

constexpr auto digit_sum(unsigned long n, unsigned long acc = 0) {
  if (n == 0) return acc;
  return digit_sum(n / 10, acc + n % 10);
}

constexpr auto gcd(int a, int b);  // RET-4: the placeholder again below
constexpr auto gcd(int a, int b) {
  if (b == 0) return a;
  return gcd(b, a % b);
}

static_assert(fact(5) == 120);
static_assert(same_type<decltype(fact(3)), int>);
static_assert(fib(25) == 75025);   // fib(40) exceeds the step limits
static_assert(same_type<decltype(fib(3)), long long>);
static_assert(digit_sum(98765) == 35);
static_assert(same_type<decltype(digit_sum(1)), unsigned long>);
static_assert(gcd(84, 36) == 12);

int main(int argc, char**) {
  int n = argc + 4;              // 5 when run plainly
  if (fact(n) != 120) return 1;
  if (fib(n * 6) != 832040) return 2;
  if (digit_sum(static_cast<unsigned long>(n) * 1111) != 20) return 3;
  if (gcd(n * 12, 18) != 6) return 4;
}
