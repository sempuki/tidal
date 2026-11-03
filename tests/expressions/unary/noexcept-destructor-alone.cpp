// tidal-rules: UN-18 UN-13
// tidal-expect: accept
// tidal-note: Write report W10. A pseudo call of the destructor through a null pointer asks about the destructor alone, and a dereferenced null pointer supplies a reference argument. Neither operand is evaluated.
struct Loud { ~Loud() noexcept(false) {} };
struct Calm { ~Calm() {} };
static_assert(!noexcept(static_cast<Loud*>(nullptr)->~Loud()));
static_assert(noexcept(static_cast<Calm*>(nullptr)->~Calm()));
void take(Loud&) noexcept;
void keep(Calm&);
static_assert(noexcept(take(*static_cast<Loud*>(nullptr))));
static_assert(!noexcept(keep(*static_cast<Calm*>(nullptr))));
