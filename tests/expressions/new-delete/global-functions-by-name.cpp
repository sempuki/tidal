// tidal-rules: NEW-19
// tidal-expect: run
// tidal-expect-clang-18: reject
// tidal-error-clang-18: no matching function for call to .operator delete
// tidal-note: Q-0125. Clang 18 does not declare the sized deallocation functions; Clang 22 does.
int main() {
  void* one = operator new(sizeof(int));
  int* i = static_cast<int*>(one);
  *i = 4;
  bool ok = *i == 4;
  operator delete(one);
  void* two = ::operator new[](8);
  ::operator delete[](two);
  void* three = operator new(16);
  operator delete(three, 16);
  void* four = operator new[](8);
  operator delete[](four, 8);
  return ok ? 0 : 1;
}
