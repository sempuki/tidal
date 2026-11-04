// tidal-rules: STG-29
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-note: The byte array is a member of Box. The cast gives a Point* that still points to the first byte (STG-29), and the pointer gives the address of the storage. std::construct_at takes that pointer as the place for the new object, and the program uses only the pointer that construct_at returns.
inline void* operator new(decltype(sizeof 0), void* where) noexcept { return where; }
namespace std {
template <class T, class... Args>
constexpr T* construct_at(T* location, Args&&... args) {
  return ::new (static_cast<void*>(location)) T(static_cast<Args&&>(args)...);
}
}
struct Point {
  int x, y;
  Point(int a, int b) : x(a), y(b) {}
};
struct Box {
  int tag = 1;
  alignas(Point) unsigned char storage[sizeof(Point)];
};
int main() {
  Box box;
  Point* made = std::construct_at(reinterpret_cast<Point*>(box.storage), 2, 5);
  bool same = static_cast<void*>(made) == static_cast<void*>(box.storage);
  int sum = made->x + made->y;
  made->~Point();
  return same && sum == 7 && box.tag == 1 ? 0 : 1;
}
