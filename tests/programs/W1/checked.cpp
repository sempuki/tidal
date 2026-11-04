// tidal-rules: TYP-4 TYP-1 PRIM-1 BIT-5 ADD-9 CONV-19 CONV-2 MUL-9 MUL-10 MUL-6 MUL-7 UN-4 SHF-7 SHF-5 ADD-5 MUL-8 TR-10
// tidal-expect: run
// tidal-note: Write suite W1, program checked.cpp (reports/clean-room/W1/report.md). The report has no construct-to-rule notes, so the rules are the ones the program's comments name.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Checked integer arithmetic that refuses to overflow.
// Each operation returns a Checked value: ok is false when the
// mathematical result does not fit in int (like Rust's checked_* -> Option).

// TYP-4, TYP-1: int has 32 bits, two's complement.
// PRIM-1: 2147483647 is an int literal. 2147483648 is a long literal,
// and the minus is a separate operator. So int_min is written as
// -2147483647 - 1, which stays in int (the MUL page does the same).
constexpr int int_max = 2147483647;
constexpr int int_min = -2147483647 - 1;
constexpr unsigned uint_max = ~0u;  // BIT-5, PRIM-1 (u suffix)

struct Checked {
  bool ok;
  int value;
};

constexpr Checked some(int v) { return Checked{true, v}; }
constexpr Checked none() { return Checked{false, 0}; }

// ADD-9: signed overflow is undefined, so the test must come before the
// addition and must itself not overflow.
constexpr Checked checked_add(int a, int b) {
  if (b > 0 && a > int_max - b) return none();
  if (b < 0 && a < int_min - b) return none();
  return some(a + b);
}

constexpr Checked checked_sub(int a, int b) {
  if (b < 0 && a > int_max + b) return none();
  if (b > 0 && a < int_min + b) return none();
  return some(a - b);
}

// CONV-19 step 3: int and long long are both signed, so the product is
// computed in long long, which holds every product of two ints (TYP-1).
constexpr Checked checked_mul(int a, int b) {
  long long wide = static_cast<long long>(a) * b;
  if (wide > int_max || wide < int_min) return none();
  return some(static_cast<int>(wide));  // fits, so CONV-2 keeps the value
}

// MUL-9: division by zero is undefined. MUL-10: int_min / -1 is undefined.
constexpr Checked checked_div(int a, int b) {
  if (b == 0) return none();
  if (a == int_min && b == -1) return none();
  return some(a / b);  // MUL-6: rounds toward zero
}

// MUL-10: int_min % -1 is undefined although the remainder would be 0.
constexpr Checked checked_rem(int a, int b) {
  if (b == 0) return none();
  if (a == int_min && b == -1) return none();
  return some(a % b);  // MUL-7: sign of a
}

// UN-4: negating int_min is undefined.
constexpr Checked checked_neg(int a) {
  if (a == int_min) return none();
  return some(-a);
}

// SHF-7: a count below 0 or at least 32 is undefined.
// SHF-5: within that range, a << n is defined for any a, modulo 2^32.
// Rust's checked_shl checks only the count, so this does the same.
constexpr Checked checked_shl(int a, int n) {
  if (n < 0 || n >= 32) return none();
  return some(a << n);
}

// Unsigned addition wraps (ADD-5), so overflow shows as a result
// smaller than an operand.
struct CheckedU {
  bool ok;
  unsigned value;
};

constexpr CheckedU checked_add_u(unsigned a, unsigned b) {
  unsigned r = a + b;
  if (r < a) return CheckedU{false, 0};
  return CheckedU{true, r};
}

// MUL-8: unsigned multiplication wraps. Test with division first.
constexpr CheckedU checked_mul_u(unsigned a, unsigned b) {
  if (a != 0 && b > uint_max / a) return CheckedU{false, 0};
  return CheckedU{true, a * b};
}

// Rust's wrapping_add for i32: compute in unsigned, convert back (CONV-2).
constexpr int wrapping_add(int a, int b) {
  return static_cast<int>(static_cast<unsigned>(a) + static_cast<unsigned>(b));
}

// Rust's saturating_add.
constexpr int saturating_add(int a, int b) {
  Checked r = checked_add(a, b);
  if (r.ok) return r.value;
  return b > 0 ? int_max : int_min;
}

constexpr bool is(Checked c, int v) { return c.ok && c.value == v; }
constexpr bool fails(Checked c) { return !c.ok; }

// Compile-time tests: constant evaluation rejects undefined behavior
// (TR-10), so every passing assert also shows no overflow happened.
static_assert(int_min == -2147483648);
static_assert(is(checked_add(1, 2), 3));
static_assert(is(checked_add(int_max - 1, 1), int_max));
static_assert(fails(checked_add(int_max, 1)));
static_assert(fails(checked_add(int_min, -1)));
static_assert(is(checked_add(int_min, int_max), -1));
static_assert(is(checked_sub(int_min + 1, 1), int_min));
static_assert(fails(checked_sub(int_min, 1)));
static_assert(fails(checked_sub(0, int_min)));
static_assert(is(checked_sub(-1, int_min), int_max));
static_assert(is(checked_mul(46340, 46340), 2147395600));
static_assert(fails(checked_mul(46341, 46341)));
static_assert(is(checked_mul(-65536, 32768), int_min));
static_assert(fails(checked_mul(int_min, -1)));
static_assert(fails(checked_div(1, 0)));
static_assert(fails(checked_div(int_min, -1)));
static_assert(is(checked_div(-7, 2), -3));
static_assert(is(checked_rem(-7, 2), -1));
static_assert(is(checked_rem(7, -2), 1));
static_assert(fails(checked_rem(int_min, -1)));
static_assert(fails(checked_neg(int_min)));
static_assert(is(checked_neg(int_max), int_min + 1));
static_assert(is(checked_shl(1, 31), int_min));
static_assert(is(checked_shl(-1, 3), -8));
static_assert(fails(checked_shl(1, 32)));
static_assert(fails(checked_shl(1, -1)));
static_assert(checked_add_u(uint_max, 0).ok);
static_assert(!checked_add_u(uint_max, 1).ok);
static_assert(checked_mul_u(65536, 65535).value == 4294901760u);
static_assert(!checked_mul_u(65536, 65536).ok);
static_assert(wrapping_add(int_max, 1) == int_min);
static_assert(saturating_add(int_max, 5) == int_max);
static_assert(saturating_add(int_min, -5) == int_min);

// Run-time tests with values the compiler cannot fold away.
int run(int argc) {
  int big = int_max - 2 + argc;  // argc is 1, so big is int_max - 1
  if (!is(checked_add(big, 1), int_max)) return 1;
  if (!fails(checked_add(big, 2))) return 2;
  if (!fails(checked_mul(big, 2))) return 3;
  if (!fails(checked_div(argc, argc - 1))) return 4;
  if (!fails(checked_neg(int_min + argc - 1))) return 5;
  if (!fails(checked_shl(argc, 31 + argc))) return 6;
  if (!fails(checked_rem(int_min, -argc))) return 7;
  if (wrapping_add(big, 3) != int_min + 1) return 8;
  int sum = 0;
  int i = 0;
  for (;; ++i) {  // add 0, 1, 2, ... until the sum would overflow
    Checked r = checked_add(sum, i * argc);
    if (!r.ok) break;
    sum = r.value;
  }
  // 0 + 1 + ... + 65535 = 2147450880 fits; adding 65536 does not.
  if (i != 65536 || sum != 2147450880) return 9;
  return 0;
}

int main(int argc, char**) { return run(argc); }
