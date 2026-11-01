// tidal-rules: LSUP-8 NEW-35
// tidal-expect: run
// tidal-expect-clang: run-fail
// tidal-note: Q-0301. Clang passes the allocation function a size it cannot meet and throws std::bad_alloc.
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
class bad_array_new_length : public bad_alloc {
public:
  bad_array_new_length() noexcept {}
  ~bad_array_new_length() override;
};
}
void* volatile kept = nullptr;
long bound(long n) { return n; }
int main() {
  try { kept = new int[bound(-1)]; return 1; }
  catch (std::bad_array_new_length&) {}
  catch (...) { return 2; }
  try { kept = new int[bound(-1)]; return 3; }
  catch (std::bad_alloc&) {}
  try { kept = new int[bound(-1)]; return 4; }
  catch (std::exception&) {}
  return 0;
}
