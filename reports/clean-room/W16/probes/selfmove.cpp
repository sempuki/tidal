struct O { int* p = nullptr; O() = default; O(O&& o) : p(o.p) { o.p = nullptr; }
  O& operator=(O&& o) { if (this != &o) { delete p; p = o.p; o.p = nullptr; } return *this; }
  ~O() { delete p; } };
int main() { O a; a = static_cast<O&&>(a); }
