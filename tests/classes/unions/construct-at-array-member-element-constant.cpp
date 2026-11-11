// tidal-rules: UNI-14 UNI-6
// tidal-expect: reject
// tidal-expect-gcc-16: accept
// tidal-expect-gcc-host: accept
// tidal-expect-edg: accept
// tidal-error-clang: construction of subobject of member .items. of union with no active member
// tidal-error-gcc: accessing uninitialized member .Row::items.
// tidal-note: W20-2, with the writer probe reports/clean-room/roadmap-10/writer-probes/w20-2-union-array-member.json. Q-0425. The union Row has no active member, and std::construct_at builds element 1 of the array member items. The new int is not of the type of items, so UNI-14 does not make items active, and the construction uses a member that does not exist (UNI-6). Clang 18 and 22 and GCC 14 reject the constant. GCC 16 and EDG accept it. The test follows Clang and GCC 14 while the question is open.
#include "../../foundations/storage/construct.h"
union Row {
  int items[2];
  constexpr Row() {}
};
constexpr int build() {
  Row r;
  int* second = std::construct_at(&r.items[1], 7);
  return *second;
}
static_assert(build() == 7);
