// W2: which operand orders are fixed. No headers.
// Each operand calls Log::mark, which records its id. Calls in two
// operands never interleave (SEQ-1, SEQ-8), so the log is well defined
// even where the order is not fixed.

struct Log {
  int seq[8]{};
  int n = 0;
  constexpr int mark(int id) { seq[n++] = id; return id; }
  // The ids in call order as one number: marks 2 then 1 give 21.
  constexpr int code() const {
    int c = 0;
    for (int i = 0; i < n; ++i) c = c * 10 + seq[i];
    return c;
  }
};

struct Pair { int a, b; };
struct Three { int a, b, c; };
constexpr int twice(int x) { return 2 * x; }
constexpr int sum2(int x, int y) { return x + y; }

// An overloaded << keeps the built-in order (SEQ-6).
struct Sink {
  int total;
  constexpr Sink operator<<(int v) const { return Sink{total * 10 + v}; }
};

// ---- Orders that the language fixes ----

constexpr int and_order() { Log L; (void)(L.mark(1) && L.mark(2)); return L.code(); }
constexpr int or_order()  { Log L; (void)(L.mark(0) || L.mark(2)); return L.code(); }
constexpr int or_skip()   { Log L; (void)(L.mark(1) || L.mark(2)); return L.code(); }
constexpr int comma_order() { Log L; (void)(L.mark(1), L.mark(2)); return L.code(); }   // COMMA-1
constexpr int cond_order(int c) {                                                      // COND-2
  Log L; (void)(L.mark(c) ? L.mark(2) : L.mark(3)); return L.code();
}
// SHF-8 holds for << and >>. The static assertion uses >>, and the run
// time check of << marks through a function that is not constexpr.
// GCC with -fsanitize=shift (and -Wall) calls a constexpr function in
// the left operand of << twice (see the W2 report).
constexpr int shift_order() { Log L; int s = L.mark(8) >> L.mark(2); (void)s; return L.code(); }
struct RunLog {
  int seq[8]{};
  int n = 0;
  int mark(int id) { seq[n++] = id; return id; }
};
int shl_order() {
  RunLog L;
  int s = L.mark(1) << L.mark(2);
  (void)s;
  return L.n == 2 ? L.seq[0] * 10 + L.seq[1] : -1;
}
constexpr int sink_order() {                                                           // SEQ-6
  Log L; Sink s = Sink{L.mark(1)} << L.mark(2); (void)s; return L.code();
}
constexpr int assign_order() {                                                         // SEQ-3
  Log L; int cells[4]{}; cells[L.mark(1)] = L.mark(2); return cells[1] == 2 ? L.code() : -1;
}
constexpr int compound_order() {                                                       // SEQ-3
  Log L; int cells[4]{}; cells[L.mark(1)] += L.mark(2); return L.code();
}
constexpr int subscript_order() {                                                      // SEQ-3
  Log L; int cells[4]{};
  int* p = cells;
  int v = (L.mark(1), p)[L.mark(2)];
  (void)v;
  return L.code();
}
constexpr int member_pointer_order() {                                                 // SEQ-3
  Log L; Pair p{5, 6};
  int Pair::* mp = &Pair::b;
  int v = (L.mark(1), p).*(L.mark(2), mp);
  (void)v;
  return L.code();
}
constexpr int callee_order() {                                                         // SEQ-2
  Log L; int (*f)(int) = twice;
  int v = (L.mark(1), f)(L.mark(2));
  (void)v;
  return L.code();
}
constexpr int braced_order() {                                                         // SEQ-3
  Log L; Three t{L.mark(1), L.mark(2), L.mark(3)}; (void)t; return L.code();
}

static_assert(and_order() == 12);
static_assert(or_order() == 2);          // mark(0) is false, so the right side runs: log 0,2
static_assert(or_skip() == 1);           // left is true, right skipped (LOG-4)
static_assert(comma_order() == 12);
static_assert(cond_order(1) == 12);      // only the selected operand runs
static_assert(cond_order(0) == 3);       // log 0,3
static_assert(shift_order() == 82);
static_assert(sink_order() == 12);
static_assert(assign_order() == 21);     // right side first
static_assert(compound_order() == 21);
static_assert(subscript_order() == 12);  // array before index
static_assert(member_pointer_order() == 12);
static_assert(callee_order() == 12);     // the function before its argument
static_assert(braced_order() == 123);

// ---- Orders that the language leaves open (SEQ-1, SEQ-8) ----
// Only "one of the two" can be tested. Each run can differ.

constexpr bool either(int code) { return code == 12 || code == 21; }

constexpr int plus_order()  { Log L; int v = L.mark(1) + L.mark(2); (void)v; return L.code(); }
constexpr int less_order()  { Log L; bool v = L.mark(1) < L.mark(2); (void)v; return L.code(); }
constexpr int args_order()  { Log L; int v = sum2(L.mark(1), L.mark(2)); (void)v; return L.code(); }
constexpr int spaceship_free_order() {
  Log L; int v = L.mark(1) * L.mark(2); (void)v; return L.code();
}

static_assert(either(plus_order()));
static_assert(either(less_order()));
static_assert(either(args_order()));
static_assert(either(spaceship_free_order()));

int main() {
  // The fixed orders hold at run time too.
  volatile int one = 1;   // keeps the calls at run time
  if (cond_order(one) != 12) return 1;
  if (assign_order() != 21 || compound_order() != 21) return 2;
  if (subscript_order() != 12 || callee_order() != 12) return 3;
  if (shift_order() != 82 || shl_order() != 12 || sink_order() != 12) return 4;
  if (braced_order() != 123 || member_pointer_order() != 12) return 5;
  if (and_order() != 12 || or_skip() != 1 || comma_order() != 12) return 6;

  // The open orders are one of two at run time.
  if (!either(plus_order()) || !either(less_order())) return 7;
  if (!either(args_order()) || !either(spaceship_free_order())) return 8;
  return 0;
}
