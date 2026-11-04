// tidal-rules: LSUP-5
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: referred to as .struct.
// tidal-error-edg: tag kind of class or struct is incompatible
// tidal-note: Q-0299. LSUP-5 needs no diagnostic. GCC and EDG reject the shape, and Clang accepts any declaration of the name.
namespace std {
struct align_val_t { decltype(sizeof 0) value; };
}
