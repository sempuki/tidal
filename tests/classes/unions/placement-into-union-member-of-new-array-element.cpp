// tidal-rules: UNI-14 STG-9
// tidal-expect: run
// tidal-note: Roadmap round 10, programmer reduction placement_new_into_union_member_of_new_array_element.cpp. Each Bucket element of an array from new[] has an anonymous union with no active member. A placement new builds an Entry at the address of the member entry of element 1, so the new object is that member (UNI-14). The array from new[] is unchanged, and delete[] frees it.
inline void* operator new(decltype(sizeof 0), void* at) noexcept { return at; }
struct Entry { int key; int value; };
struct Bucket {
  union { Entry entry; };
  bool full = false;
  Bucket() {}
};
int main() {
  Bucket* table = new Bucket[3];
  ::new (static_cast<void*>(&table[1].entry)) Entry{4, 40};
  table[1].full = true;
  int result = (table[1].full && table[1].entry.value == 40 && !table[0].full) ? 0 : 1;
  delete[] table;
  return result;
}
