// tidal-rules: TYN-6
// tidal-expect: accept
// tidal-note: Ledger row RV9-R-4. A partial specialization of a class template that is declared without its body can have attributes and an alignment specifier after the class key (specialization_class). The later definitions complete the specializations. Every compiler accepts the program.
template <class T> struct Slot;
template <class T> struct [[deprecated]] Slot<T*>;
template <class T> struct Slot<T*> { T* target; };
template <class T> struct alignas(16) Slot<T&>;
template <class T> struct alignas(16) Slot<T&> { T* target; };
static_assert(alignof(Slot<int&>) == 16);
