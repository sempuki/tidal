// tidal-rules: EXS-9
// tidal-expect: accept
template <class T> void store(T) noexcept(sizeof(T) > 1) {}
static_assert(!noexcept(store('a')));
static_assert(noexcept(store(1.0)));
void (*fast)(double) noexcept = &store<double>;

struct Quiet { Quiet() = default; Quiet(const Quiet&) noexcept {} };
struct Loud { Loud() = default; Loud(const Loud&) {} };
template <class T> T copy_of(const T& from) noexcept(noexcept(T(from))) { return from; }
Quiet quiet;
Loud loud;
static_assert(noexcept(copy_of(quiet)));
static_assert(!noexcept(copy_of(loud)));
