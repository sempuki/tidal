// Two objects built in two rows of one byte array. A pointer to the
// first object does not reach the second by arithmetic.
// Front end: undefined [PFX-4] at line 13: subscript outside the array.
// GCC 16 and Clang 22, also with -fsanitize=undefined,address, exit 0.
namespace std { using size_t = decltype(sizeof 0); }
inline void* operator new(std::size_t, void* at) noexcept { return at; }

alignas(int) unsigned char rows[2][sizeof(int)];

int main() {
  int* a = new (rows[0]) int(1);
  new (rows[1]) int(2);
  return a[1] == 2 ? 0 : 1;   // the front end stops here
}
