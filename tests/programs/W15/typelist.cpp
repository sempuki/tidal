// tidal-rules: LSUP-4 LSUP-7 LSUP-10 TR-2 TR-5 TR-10 CE-1 TPL-30 PACK-19 TSP-9 TSP-14 TSP-15 ORD-14 TPL-12 TARG-7 TARG-17 PRIM-28
// tidal-expect: run
// tidal-note: Write suite W15, program typelist.cpp (reports/clean-room/W15/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// W15 exercise 4: a compile-time list of types with length, index of a
// type, and "every type has a property". Written from the Tidal
// specification by a clean-room reader.
//
// Rules used:
//   TPL-1, TPL-30           class template with a type pack, last in the head
//   TSP-9, TSP-14, TSP-15   partial specializations, matched by deduction;
//                           the most specialized one wins
//   ORD-14                  IndexOf<T, TypeList<T, Rest...>> is more
//                           specialized than IndexOf<T, TypeList<U, Rest...>>
//   PACK-19                 sizeof...(Ts), of the type of sizeof
//   PRIM-27, PRIM-28        the fold (P<Ts>::value && ...); an empty pack
//                           gives true
//   TPL-12                  a template template parameter takes the property;
//                           its argument is a class template or an alias
//                           template, so a concept is wrapped in a class
//   TPL-26, TPL-28          variable templates and alias templates as the
//                           front-end names
//   CPT-1, CPT-17           a concept over the list, and a constrained
//                           template that uses it

using size_t = decltype(sizeof 0);

template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;

// The list. It has no members; its arguments are the data.
template <class... Ts>
struct TypeList {};

// ---- length ---------------------------------------------------------------

template <class L> struct Length;
template <class... Ts>
struct Length<TypeList<Ts...>> {
  static constexpr size_t value = sizeof...(Ts);
};
template <class L>
constexpr size_t length = Length<L>::value;

// ---- index of a type ------------------------------------------------------
// value is the position of the first T, or -1 when the list has no T.

template <class T, class L> struct IndexOf;

template <class T>
struct IndexOf<T, TypeList<>> {
  static constexpr int value = -1;
};

// The head is T. This one is more specialized than the next, so it wins
// when both match.
template <class T, class... Rest>
struct IndexOf<T, TypeList<T, Rest...>> {
  static constexpr int value = 0;
};

template <class T, class U, class... Rest>
struct IndexOf<T, TypeList<U, Rest...>> {
 private:
  static constexpr int tail = IndexOf<T, TypeList<Rest...>>::value;

 public:
  static constexpr int value = tail < 0 ? -1 : tail + 1;
};

template <class T, class L>
constexpr int index_of = IndexOf<T, L>::value;

// ---- every type has a property -------------------------------------------
// A property is a class template P with a bool constant P<T>::value.

template <template <class> class P, class L> struct AllOf;
template <template <class> class P, class... Ts>
struct AllOf<P, TypeList<Ts...>> {
  static constexpr bool value = (P<Ts>::value && ...);
};
template <template <class> class P, class L>
constexpr bool all_of = AllOf<P, L>::value;

// ---- a few more list operations, to use the others -----------------------

template <class L, class T> struct PushFront;
template <class... Ts, class T>
struct PushFront<TypeList<Ts...>, T> {
  using type = TypeList<T, Ts...>;
};
template <class L, class T>
using push_front = typename PushFront<L, T>::type;

template <class A, class B> struct Concat;
template <class... As, class... Bs>
struct Concat<TypeList<As...>, TypeList<Bs...>> {
  using type = TypeList<As..., Bs...>;
};
template <class A, class B>
using concat = typename Concat<A, B>::type;

// ---- properties -----------------------------------------------------------

template <class T>
struct IsSmall {
  static constexpr bool value = sizeof(T) <= 4;
};

template <class T>
concept Integral =
    same<T, bool> || same<T, char> || same<T, short> || same<T, int> ||
    same<T, long> || same<T, long long> || same<T, unsigned> ||
    same<T, unsigned long>;

// A concept cannot be the argument of a template template parameter
// (TPL-12 names class templates and alias templates), so a class carries
// the concept's value.
template <class T>
struct IsIntegral {
  static constexpr bool value = Integral<T>;
};

// A list concept, and a constrained template that uses it.
template <class L>
concept AllIntegral = all_of<IsIntegral, L>;

template <class L>
  requires AllIntegral<L>
constexpr size_t integral_count() {
  return length<L>;
}

// ---- checks ---------------------------------------------------------------

using Empty = TypeList<>;
using Ints = TypeList<char, short, int, long>;
using Mixed = TypeList<int, double, char, double>;

static_assert(length<Empty> == 0);
static_assert(length<Ints> == 4);
static_assert(length<concat<Ints, Mixed>> == 8);
static_assert(same<push_front<Empty, int>, TypeList<int>>);
static_assert(same<concat<TypeList<int>, TypeList<char>>, TypeList<int, char>>);

static_assert(index_of<int, Mixed> == 0);
static_assert(index_of<double, Mixed> == 1);   // the first double
static_assert(index_of<char, Mixed> == 2);
static_assert(index_of<float, Mixed> == -1);
static_assert(index_of<int, Empty> == -1);
static_assert(index_of<long, Ints> == 3);

static_assert(all_of<IsSmall, Ints> == false);  // long is 8 bytes on LP64
static_assert(all_of<IsSmall, TypeList<char, int>>);
static_assert(all_of<IsSmall, Empty>);          // empty fold over && is true
static_assert(all_of<IsIntegral, Ints>);
static_assert(!all_of<IsIntegral, Mixed>);

static_assert(AllIntegral<Ints> && !AllIntegral<Mixed>);
static_assert(integral_count<Ints>() == 4);

template <class L>
concept Countable = requires { integral_count<L>(); };
static_assert(Countable<Ints> && !Countable<Mixed>);

int main() {
  return index_of<long, Ints> == 3 && length<Mixed> == 4 ? 0 : 1;
}
