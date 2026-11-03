// tidal-rules: AUTO-17
// tidal-expect: reject
// tidal-error-clang: non-const static data member must be initialized out of line
// tidal-error-gcc: in-class initialization of non-const static member
// tidal-error-edg: member with an in-class initializer must be const
// tidal-note: The static data member has an initializer, but it is neither const nor inline.
struct S { static auto n = 1; };
