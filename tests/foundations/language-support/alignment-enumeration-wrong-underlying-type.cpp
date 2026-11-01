// tidal-rules: LSUP-5
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: different underlying type in enum
// tidal-error-edg: was previously declared with
// tidal-note: Q-0299. Clang accepts any declaration of the name.
namespace std {
enum class align_val_t : unsigned char {};
}
