// Q-0421 with the library's own std::allocator and std::construct_at.
#include <memory>
struct Pod { int v; };
constexpr int assign_int() {
  std::allocator<int> a; int* p = a.allocate(2); p[1] = 7; int v = p[1]; a.deallocate(p, 2); return v;
}
constexpr int assign_member() {
  std::allocator<Pod> a; Pod* p = a.allocate(2); p[1].v = 7; int v = p[1].v; a.deallocate(p, 2); return v;
}
constexpr int construct_first() {
  std::allocator<int> a; int* p = a.allocate(2); std::construct_at(p + 1, 7); int v = p[1]; a.deallocate(p, 2); return v;
}
#ifdef ASSIGN_INT
static_assert(assign_int() == 7);
#endif
#ifdef ASSIGN_MEMBER
static_assert(assign_member() == 7);
#endif
static_assert(construct_first() == 7);
int main() { return 0; }
