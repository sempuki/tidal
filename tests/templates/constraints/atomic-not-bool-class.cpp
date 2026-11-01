// tidal-rules: CNS-9
// tidal-expect: reject
// tidal-error-clang: atomic constraint must be of type 'bool'
// tidal-error-gcc: not .bool.|does not have type .bool.
// tidal-error-edg: atomic constraint must have type bool
// tidal-note: A class with a conversion to bool is not enough.
struct Truthy { constexpr operator bool() const { return true; } };
template <class T> struct Wrap { static constexpr Truthy value{}; };
template <class T> requires (Wrap<T>::value) void use(T) {}
void run() { use(1); }
