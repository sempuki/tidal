// tidal-rules: NEW-28 OVL-35
// tidal-expect: reject
// tidal-error-clang: no matching function for call to .operator new.
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: cannot be called with the given argument list
// tidal-note: Decision 0059, item RV5, point RV5-B-4. The class declares only an aligned allocation form, and the type is not over-aligned. The new expression passes no alignment, finds no form, and is rejected on every baseline compiler. The review reads the C++23 text as adding the alignment on a second search (documentation/decisions/pending.md, row 4).
namespace std { enum class align_val_t : decltype(sizeof 0) {}; }
struct Aligned {
  int v = 0;
  static void* operator new(decltype(sizeof 0), std::align_val_t);
  static void operator delete(void*, std::align_val_t);
};
Aligned* make() { return new Aligned; }
