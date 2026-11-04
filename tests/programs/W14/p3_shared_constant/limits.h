// limits.h: constants that several units share.
#ifndef W14_P3_LIMITS_H
#define W14_P3_LIMITS_H

namespace limits {

// One object for the whole program: an inline variable has one address
// (LNK-4, ODR-7). `inline constexpr` keeps external linkage (INL-1, LNK-3).
inline constexpr int max_users = 64;

// A plain constexpr variable has internal linkage (LNK-3): each unit
// that includes this header gets its own object with the same value.
constexpr int page_size = 16;

// A shared constant whose value lives in one unit (like an Ada deferred
// constant). `extern` keeps external linkage (LNK-3); config.cpp
// defines it. Other units see only the declaration, so its value is not
// a constant expression for them.
extern const int build_number;

// Functions in units a.cpp and b.cpp report the addresses they see.
const int* max_users_seen_by_a();
const int* page_size_seen_by_a();
int buffer_cells_in_b();

}  // namespace limits

#endif
