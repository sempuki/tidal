struct S {
  template <class T> operator T*() const { return nullptr; }
  template <class T> operator T* const() const { return nullptr; }
};
int* p = S{};
