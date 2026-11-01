// tidal-rules: TARG-19
// tidal-expect: accept
// tidal-note: Two enumerators with one value, unions with one active member and value, and class arguments with array members.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
enum Level { low = 1, also_low = 1, high = 2 };
template <Level L> struct Tag {};
static_assert(same<Tag<low>, Tag<also_low>>);
static_assert(!same<Tag<low>, Tag<high>>);
union Choice { int i; char c; };
template <Choice C> struct Pick {};
static_assert(same<Pick<Choice{.i = 1}>, Pick<Choice{.i = 1}>>);
static_assert(!same<Pick<Choice{.i = 1}>, Pick<Choice{.c = 1}>>);
struct Pair { int v[2]; };
template <Pair P> struct Hold {};
static_assert(same<Hold<Pair{{1, 2}}>, Hold<Pair{{1, 2}}>>);
static_assert(!same<Hold<Pair{{1, 2}}>, Hold<Pair{{2, 1}}>>);
