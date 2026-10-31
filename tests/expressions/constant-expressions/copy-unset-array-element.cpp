// tidal-rules: CE-22 OBJ-2
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: subobject .data. is not initialized
// tidal-error-edg: access to uninitialized object
// tidal-note: Q-0115. Clang and EDG reject the copy because it reads the array elements that have no value; GCC accepts it.
struct Ring {
  int data[4];
  int size = 0;
};
constexpr Ring make_ring() {
  Ring ring;
  ring.size = 1;
  return ring;
}
constexpr int size = make_ring().size;
