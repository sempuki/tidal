// tidal-rules: TYN-6
// tidal-expect: accept
// tidal-note: Ledger row RM12-2. A forward declaration can have attributes and an alignment specifier after the class key (forward_class). The definition repeats the alignment specifier, which Clang and EDG require. Every compiler accepts both forms.
struct [[maybe_unused]] Marked;
struct alignas(8) Aligned;
struct alignas(8) Aligned { int value; };
static_assert(alignof(Aligned) == 8);
