// tidal-rules: NEW-42 NEW-38
// tidal-expect: run
// tidal-note: A request for 2^47 bytes or more exceeds the storage that an x86-64 process can map, so the global allocation functions throw.
namespace std {
class exception {
public:
  exception() noexcept {}
  virtual ~exception();
};
class bad_alloc : public exception {
public:
  bad_alloc() noexcept {}
  ~bad_alloc() override;
};
}
using size = decltype(sizeof 0);
void* volatile kept = nullptr;
size volatile limit = size{1} << 47;
int main() {
  try {
    kept = operator new(limit);
    return 1;
  } catch (std::bad_alloc&) {
  }
  try {
    kept = new char[limit + 1];
    return 2;
  } catch (std::bad_alloc&) {
  }
  try {
    kept = operator new[](limit * 2);
    return 3;
  } catch (std::bad_alloc&) {
  }
  return 0;
}
