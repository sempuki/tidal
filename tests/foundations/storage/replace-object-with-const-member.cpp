// tidal-rules: STG-9
// tidal-expect: run
#include "placement.h"
struct Entry { const int key; int value; };
int read_key(const Entry& entry) { return entry.key; }
int main() {
  Entry entry{1, 2};
  int (*volatile reader)(const Entry&) = read_key;
  int before = reader(entry);
  entry.~Entry();
  new (&entry) Entry{7, 8};
  return before == 1 && entry.key == 7 && reader(entry) == 7 && entry.value == 8 ? 0 : 1;
}
