// tidal-rules: ICS-17 ICS-25
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: ambiguous
// tidal-note: Q-0419. Decision 0059, item F2 (I3-13). A braced list for references to arrays of int and of long. The first element converts by Q to long, and the second is the int 0. The worst kind is user-defined, so the list sequence holds Q's conversion, and the long form wins. Clang and EDG choose it; GCC calls the call ambiguous.
struct Q { constexpr operator long() const { return 2; } };
constexpr int f(const int (&)[2]) { return 1; }
constexpr int f(const long (&)[2]) { return 2; }
static_assert(f({Q{}, 0}) == 2);
