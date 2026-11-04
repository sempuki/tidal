// tidal-rules: TYP-1 CONV-1 PRIM-7 UN-20 ADD-5 MUL-8 PRIM-3 CONV-2 UN-3 TR-9
// tidal-expect: run
// tidal-note: Write suite W1, program parse.cpp (reports/clean-room/W1/report.md). The report has no construct-to-rule notes, so the rules are the ones the program's comments name.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// A parser for decimal and hexadecimal digit strings.
// Like Rust's u32::from_str_radix and str::parse::<i32>: the whole
// string must be digits, and a value that does not fit is an error.

using u32 = unsigned int;  // TYP-1: 4 bytes
constexpr u32 u32_max = ~0u;

enum class Error { None, Empty, BadDigit, Overflow };

struct ParsedU {
  Error error;
  u32 value;
};

struct ParsedI {
  Error error;
  int value;
};

// The value of one digit character, or -1. This assumes that '0'..'9'
// are consecutive, and that 'a'..'f' and 'A'..'F' are consecutive. The
// specification does not state the encoding of character literals; the
// matrix targets use ASCII (outside knowledge).
constexpr int digit_value(char c) {
  if (c >= '0' && c <= '9') return c - '0';  // CONV-1: both promote to int
  if (c >= 'a' && c <= 'f') return c - 'a' + 10;
  if (c >= 'A' && c <= 'F') return c - 'A' + 10;
  return -1;
}

// Parses digits from s up to the terminating null (PRIM-7).
constexpr ParsedU parse_digits(const char* s, u32 radix) {
  if (*s == '\0') return {Error::Empty, 0};
  u32 value = 0;
  for (; *s != '\0'; ++s) {  // UN-20: ++ on a pointer moves to the next element
    int d = digit_value(*s);
    if (d < 0 || static_cast<u32>(d) >= radix) return {Error::BadDigit, 0};
    u32 ud = static_cast<u32>(d);
    // ADD-5, MUL-8: unsigned arithmetic wraps silently, so test first.
    if (value > (u32_max - ud) / radix) return {Error::Overflow, 0};
    value = value * radix + ud;
  }
  return {Error::None, value};
}

constexpr ParsedU parse_u32(const char* s, u32 radix) { return parse_digits(s, radix); }

// "0x" or "0X" selects hexadecimal, as in a literal (PRIM-3).
// Anything else is decimal. No octal: a leading 0 is just a 0.
constexpr ParsedU parse_auto(const char* s) {
  if (s[0] == '0' && (s[1] == 'x' || s[1] == 'X')) return parse_digits(s + 2, 16);
  return parse_digits(s, 10);
}

// A signed parse with an optional sign. The magnitude is parsed as
// unsigned. -2147483648 has a magnitude of 2147483648, which fits in
// u32 but not in int, so the negation happens in u32 and the result
// converts to int (CONV-2 keeps the low 32 bits, two's complement).
constexpr ParsedI parse_i32(const char* s) {
  bool negative = false;
  if (*s == '+' || *s == '-') {
    negative = *s == '-';
    ++s;
  }
  ParsedU m = parse_auto(s);
  if (m.error != Error::None) return {m.error, 0};
  constexpr u32 limit_pos = 2147483647u;
  constexpr u32 limit_neg = 2147483648u;
  if (m.value > (negative ? limit_neg : limit_pos)) return {Error::Overflow, 0};
  u32 bits = negative ? 0u - m.value : m.value;  // UN-3, ADD-5
  return {Error::None, static_cast<int>(bits)};
}

constexpr bool ok_u(ParsedU p, u32 v) { return p.error == Error::None && p.value == v; }
constexpr bool ok_i(ParsedI p, int v) { return p.error == Error::None && p.value == v; }

static_assert(ok_u(parse_u32("0", 10), 0));
static_assert(ok_u(parse_u32("12345", 10), 12345));
static_assert(ok_u(parse_u32("4294967295", 10), 4294967295u));
static_assert(parse_u32("4294967296", 10).error == Error::Overflow);
static_assert(parse_u32("99999999999", 10).error == Error::Overflow);
static_assert(parse_u32("", 10).error == Error::Empty);
static_assert(parse_u32("12a", 10).error == Error::BadDigit);
static_assert(parse_u32("-1", 10).error == Error::BadDigit);
static_assert(ok_u(parse_u32("ff", 16), 255));
static_assert(ok_u(parse_u32("DeadBeef", 16), 0xDEADBEEFu));
static_assert(ok_u(parse_u32("FFFFFFFF", 16), u32_max));
static_assert(parse_u32("100000000", 16).error == Error::Overflow);
static_assert(parse_u32("fg", 16).error == Error::BadDigit);
static_assert(ok_u(parse_u32("101", 2), 5));
static_assert(ok_u(parse_auto("0x1F"), 31));
static_assert(ok_u(parse_auto("0X1f"), 31));
static_assert(ok_u(parse_auto("017"), 17));  // decimal, not octal
static_assert(parse_auto("0x").error == Error::Empty);
static_assert(ok_i(parse_i32("-2147483648"), -2147483647 - 1));
static_assert(ok_i(parse_i32("2147483647"), 2147483647));
static_assert(ok_i(parse_i32("+42"), 42));
static_assert(ok_i(parse_i32("-0x10"), -16));
static_assert(parse_i32("2147483648").error == Error::Overflow);
static_assert(parse_i32("-2147483649").error == Error::Overflow);
static_assert(parse_i32("-").error == Error::Empty);
static_assert(parse_i32("--1").error == Error::BadDigit);

int main(int argc, char** argv) {
  // A run-time string: the program name is not a number (TR-9).
  if (parse_auto(argv[0]).error == Error::None) return 1;

  // Build "4294967295" in a local array and parse it at run time.
  char text[16] = {};
  const char* src = "4294967295";
  int n = 0;
  while (src[n] != '\0') {
    text[n] = src[n];
    ++n;
  }
  if (!ok_u(parse_u32(text, 10), u32_max)) return 2;
  text[n - argc] = '6';  // "4294967296"
  if (parse_u32(text, 10).error != Error::Overflow) return 3;

  // Round trip: format a number in hex, parse it back.
  u32 x = 0x1234ABCDu + static_cast<u32>(argc) - 1;
  char hex[9] = {};
  const char* hex_digits = "0123456789abcdef";
  for (int i = 7; i >= 0; --i) {
    hex[i] = hex_digits[x & 0xFu];
    x >>= 4;
  }
  if (!ok_u(parse_u32(hex, 16), 0x1234ABCDu)) return 4;
  return 0;
}
