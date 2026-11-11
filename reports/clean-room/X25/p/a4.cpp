// Another member is active. std::construct_at builds element 1 of the
// array member, which does not overlap the active member. Then the
// program reads the old member.
#include "construct.h"
struct Two { int first; };
union U { Two head; int items[2]; constexpr U() : head{3} {} };
constexpr int f() {
  U u;
  std::construct_at(&u.items[1], 7);
#if CASE == 1
  return u.head.first;          // the old member
#else
  return u.items[1];            // the new element, through the member
#endif
}
#ifdef RUN
int g(int c) { U u; std::construct_at(&u.items[1], 7); return c ? u.head.first : u.items[1]; }
int main(int argc, char**) { return g(argc > 1) == 7 ? 0 : 1; }
#else
static_assert(f() > 0);
int main() {}
#endif
