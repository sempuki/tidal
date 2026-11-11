// The array member becomes active by assignments to both elements
// (UNI-13). An explicit destructor call then ends element 1. The value
// of the constant has an element of an active array member that is not
// in its lifetime.
using Int = int;
union Row { int items[2]; constexpr Row() {} };
constexpr Row make() {
  Row r;
  r.items[0] = 1;
  r.items[1] = 2;
#ifdef END
  r.items[1].~Int();
#endif
  return r;
}
constexpr Row kept = make();
static_assert(kept.items[0] == 1);
int main() {}
