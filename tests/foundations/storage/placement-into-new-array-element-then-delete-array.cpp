// tidal-rules: STG-9
// tidal-expect: run
// tidal-note: Roadmap round 10, programmer reduction placement_new_into_new_array_element_then_delete.cpp. A placement new builds an int in the storage of element 1 of an array from new[]. The new int replaces the element (STG-9), so the pointer still points to the array from new[], and delete[] frees that array.
inline void* operator new(decltype(sizeof 0), void* at) noexcept { return at; }
int main() {
  int* row = new int[3]{1, 2, 3};
  ::new (static_cast<void*>(&row[1])) int(20);
  int result = (row[0] == 1 && row[1] == 20 && row[2] == 3) ? 0 : 1;
  delete[] row;
  return result;
}
