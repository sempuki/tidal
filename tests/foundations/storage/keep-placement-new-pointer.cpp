// tidal-rules: STG-5
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=undefined -fno-sanitize-recover=undefined
// tidal-note: The class keeps the pointer that placement new returns and reaches the object only through it, so no launder is needed (STG-19).
#include "placement.h"
struct Reading {
  int value;
  explicit Reading(int v) : value(v) {}
  ~Reading() { value = -1; }
};
template <class T>
class Maybe {
  alignas(T) unsigned char storage[sizeof(T)];
  T* held = nullptr;
public:
  Maybe() = default;
  Maybe(const Maybe&) = delete;
  Maybe& operator=(const Maybe&) = delete;
  ~Maybe() { reset(); }
  template <class... Args>
  void emplace(Args&&... args) {
    reset();
    held = new (storage) T(static_cast<Args&&>(args)...);
  }
  void reset() {
    if (held) {
      held->~T();
      held = nullptr;
    }
  }
  T* get() const { return held; }
};
int main() {
  Maybe<Reading> slot;
  slot.emplace(4);
  int first = slot.get()->value;
  slot.emplace(9);
  int second = slot.get()->value;
  slot.reset();
  bool empty = slot.get() == nullptr;
  return first == 4 && second == 9 && empty ? 0 : 1;
}
