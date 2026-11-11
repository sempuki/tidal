// Planted bug: a member is read after an explicit destructor call ended the object.
inline void* operator new(decltype(sizeof 0), void* p) noexcept { return p; }
struct Conn { int fd = 3; ~Conn() {} };
int main() {
  Conn c;
  c.~Conn();
  int fd = c.fd;
  ::new (static_cast<void*>(&c)) Conn();
  return fd == 3 ? 0 : 1;
}
