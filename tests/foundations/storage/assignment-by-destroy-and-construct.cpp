// tidal-rules: STG-9
// tidal-expect: run
#include "placement.h"
struct Setting {
  const int id;
  int level;
  Setting& operator=(const Setting& other) {
    if (this != &other) {
      this->~Setting();
      new (this) Setting(other);
    }
    return *this;
  }
};
int main() {
  Setting current{1, 10};
  Setting wanted{2, 20};
  current = wanted;
  return current.id == 2 && current.level == 20 ? 0 : 1;
}
