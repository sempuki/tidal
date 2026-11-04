// tidal-rules: LSUP-5
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: scoped/unscoped mismatch in enum
// tidal-error-edg: was previously declared as a
// tidal-note: Q-0299. LSUP-5 needs no diagnostic. GCC and EDG reject the shape, and Clang accepts any declaration of the name.
namespace std {
using size_t = decltype(sizeof 0);
enum align_val_t : size_t {};
}
