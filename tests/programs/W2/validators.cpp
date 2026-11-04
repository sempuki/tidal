// tidal-rules: LOG-4 LOG-5 LOG-7 LOG-14 LOG-2 LOG-3 LOG-6 LOG-8 LOG-11 LOG-13
// tidal-expect: run
// tidal-note: Write suite W2, program validators.cpp (reports/clean-room/W2/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// W2: a short-circuit validator chain. No headers.

// An input record. The name may be null.
struct Form {
  const char* name;
  int age;
  int quota;
  int used;
};

constexpr int length(const char* s) {
  int n = 0;
  while (s[n] != '\0') ++n;
  return n;
}

// Each check reads only what the earlier checks made safe (LOG-4, LOG-5).
// s->... runs only when s is not null; length(f.name) only when name is
// not null; the division only when quota is not zero.
constexpr bool valid(const Form* f) {
  return f != nullptr
      && f->name                         // a pointer converts to bool (LOG-2)
      && length(f->name) > 0
      && f->age >= 0 && f->age < 150
      && f->quota != 0
      && f->used * 100 / f->quota <= 100;
}

constexpr Form ok{"ada", 36, 10, 5};
constexpr Form no_name{nullptr, 36, 10, 5};
constexpr Form empty_name{"", 36, 10, 5};
constexpr Form zero_quota{"bob", 20, 0, 0};
constexpr Form over{"cy", 20, 4, 5};

static_assert(valid(&ok));
static_assert(!valid(nullptr));          // f->name never read
static_assert(!valid(&no_name));         // length(nullptr) never called
static_assert(!valid(&empty_name));
static_assert(!valid(&zero_quota));      // used * 100 / 0 never evaluated (LOG-7)
static_assert(!valid(&over));

// || as a fallback: the right operand runs only when the left is false.
constexpr bool exempt_or_valid(const Form* f, bool exempt) {
  return exempt || valid(f);
}
static_assert(exempt_or_valid(nullptr, true));
static_assert(!exempt_or_valid(nullptr, false));

// && inside || written with parentheses (LOG-6, LOG-8).
constexpr bool admit(bool staff, bool badge, bool escorted) {
  return staff || (badge && escorted);
}
static_assert(admit(true, false, false));
static_assert(admit(false, true, true));
static_assert(!admit(false, true, false));

// A class with an explicit operator bool is a valid operand (LOG-2).
struct Result {
  int code;
  constexpr explicit operator bool() const { return code == 0; }
};
constexpr Result step(int c) { return Result{c}; }
static_assert(step(0) && step(0));
static_assert(!(step(0) && step(3)));
static_assert(step(3) || step(0));
static_assert(not step(1) and not step(2));   // word spellings (LOG-13)

// Count which checks ran, at run time.
int calls = 0;
bool check(bool v) { ++calls; return v; }

// An overloaded && is a function call: both operands run (LOG-11).
struct Flag { bool v; };
bool operator&&(Flag a, Flag b) { return a.v && b.v; }
Flag flag(bool v) { ++calls; return Flag{v}; }

int main() {
  calls = 0;
  bool r = check(false) && check(true) && check(true);
  if (r || calls != 1) return 1;         // stops at the first false

  calls = 0;
  r = check(true) || check(false) || check(false);
  if (!r || calls != 1) return 2;        // stops at the first true

  calls = 0;
  r = check(true) && check(true) && check(false) && check(true);
  if (r || calls != 3) return 3;

  // The left side is complete, side effects and all, before the right
  // side starts (LOG-5).
  int i = 0;
  r = (i++ == 0) && (i == 1);
  if (!r) return 4;

  calls = 0;
  r = flag(false) && flag(true);
  if (r || calls != 2) return 5;         // no short circuit for the overload

  Form f{"eve", 40, 8, 2};
  if (!valid(&f)) return 6;
  f.quota = 0;
  if (valid(&f)) return 7;               // no division by zero at run time
  return 0;
}
