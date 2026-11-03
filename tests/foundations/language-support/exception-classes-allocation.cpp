// tidal-rules: LSUP-8 NEW-38 NEW-42
// tidal-expect: run
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
void* volatile kept = nullptr;
decltype(sizeof 0) size(decltype(sizeof 0) n) { return n; }
int main() {
  try {
    kept = operator new(decltype(sizeof 0)(-1) / 2);
    return 1;
  } catch (std::bad_alloc&) {
  }
  try {
    kept = new char[size(decltype(sizeof 0)(-1) / 4)];
    return 2;
  } catch (std::exception&) {
  }
  return 0;
}
