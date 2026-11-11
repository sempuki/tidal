// tidal-rules: TYN-6
// tidal-expect: accept
// tidal-note: Ledger row RV9-R-4. An explicit specialization of a class template that is declared without its body can have attributes and an alignment specifier after the class key (specialization_class), also with a qualified name. The later definitions complete the specializations. Every compiler accepts the program.
template <class T> struct Slot;
template <> struct [[deprecated]] Slot<int>;
template <> struct Slot<int> { int value; };
template <> struct alignas(8) Slot<char>;
template <> struct alignas(8) Slot<char> { char value; };
static_assert(alignof(Slot<char>) == 8);
namespace store { template <class T> struct Cell; }
template <> struct [[maybe_unused]] store::Cell<long>;
