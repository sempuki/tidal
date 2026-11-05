struct S {
  template <class T> operator T*() const { return nullptr; }
  template <class T> requires (sizeof(T) > 1) operator T* const() const { return nullptr; }
};
int* p = S{};
