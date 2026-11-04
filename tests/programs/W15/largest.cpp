// tidal-rules: LSUP-4 LSUP-7 LSUP-10 TR-2 TR-5 TR-10 CE-1 TPL-30 PACK-5 PACK-9 CPT-17 CPT-19 CONV-19 DTY-1 COND-3 RET-1 TPL-26 TSP-10
// tidal-expect: run
// tidal-note: Write suite W15, program largest.cpp (reports/clean-room/W15/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// W15 exercise 2: the largest of any number of arithmetic arguments, with
// the result type that the usual arithmetic conversions give. Written from
// the Tidal specification by a clean-room reader.
//
// Rules used:
//   TPL-26, TSP-10   variable template `same` and its partial specialization
//   CPT-1, CPT-2     concepts OneOf and Arithmetic; a fold in a concept
//   PRIM-27, PRIM-28 the fold (same<T, Us> || ...)
//   CPT-17, CPT-19   `Arithmetic A` and `Arithmetic... Rest`; the pack
//                    constraint checks each element, and an empty pack
//                    satisfies it
//   TPL-1, TPL-22    two function templates named largest
//   DTY-1, CONV-19   decltype(A() + B()) names the common type of the
//                    usual arithmetic conversions; the sum is a prvalue,
//                    so decltype gives the bare type
//   COND-3           x < y ? y : x with two lvalues of one type
//   RET-1            the `auto` return type comes from the return statement
//   CMP-4            the comparison is done after both sides convert to
//                    the common type, so no sign-compare warning arises

template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;

template <class T, class... Us>
concept OneOf = (same<T, Us> || ...);

template <class T>
concept Arithmetic =
    OneOf<T, bool, char, signed char, unsigned char, wchar_t, char8_t,
          char16_t, char32_t, short, unsigned short, int, unsigned int, long,
          unsigned long, long long, unsigned long long, float, double,
          long double>;

// The common type of two arithmetic types, by the usual arithmetic
// conversions (CONV-19). The operands of + go through those conversions,
// and decltype names the type of the sum.
template <Arithmetic A, Arithmetic B>
using Common = decltype(A() + B());

// One argument: the argument itself. A single argument keeps its own
// type, with no promotion.
template <Arithmetic A>
constexpr A largest(A a) {
  return a;
}

// Two or more: fold the first two into their common type, then recurse.
template <Arithmetic A, Arithmetic B, Arithmetic... Rest>
constexpr auto largest(A a, B b, Rest... rest) {
  using C = Common<A, B>;
  C x = static_cast<C>(a);
  C y = static_cast<C>(b);
  return largest(x < y ? y : x, rest...);
}

// Result types follow the usual arithmetic conversions.
static_assert(same<decltype(largest(1, 2)), int>);
static_assert(same<decltype(largest(1, 2.5)), double>);
static_assert(same<decltype(largest(1, 2.5f)), float>);
static_assert(same<decltype(largest(short(1), char(2))), int>);  // promotion
static_assert(same<decltype(largest(1, 2L, 3u)), long>);         // LP64
static_assert(same<decltype(largest(1, 2u)), unsigned>);
static_assert(same<decltype(largest('a')), char>);               // one argument

// Values.
static_assert(largest(3, 9, 4) == 9);
static_assert(largest(1, 2.5, 'c') == 99.0);                     // 'c' is 99
static_assert(largest(-7) == -7);
static_assert(largest(2.5f, 1.25, 3) == 3.0);
static_assert(largest(true, false) == 1);
// -1 converts to the largest unsigned value before the comparison.
static_assert(largest(-1, 1u) == 4294967295u);

// A type that is not arithmetic is not accepted.
struct Meters { double v; };
template <class... Ts>
concept HasLargest = requires (Ts... ts) { largest(ts...); };
static_assert(HasLargest<int, double>);
static_assert(!HasLargest<int, Meters>);
static_assert(!HasLargest<>);                                    // no arguments

int main(int argc, char**) {
  // The same function at run time, with a value the translator cannot see.
  auto r = largest(argc, 0.5, 'A');                              // double
  static_assert(same<decltype(r), double>);
  return r == 65.0 ? 0 : 1;
}
