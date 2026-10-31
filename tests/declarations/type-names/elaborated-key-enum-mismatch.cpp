// tidal-rules: TYN-4
// tidal-expect: reject
// tidal-error-clang: tag type that does not match previous declaration
// tidal-error-gcc: referred to as .struct
// tidal-error-edg: is incompatible with declaration of enum
enum E { a };
struct E* p;
