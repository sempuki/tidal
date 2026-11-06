// Two specializations of a variable template have two addresses, and
// CMP-15 lets constant evaluation compare them. GCC 16 accepts this at
// -std=c++23 -pedantic-errors and rejects it ("non-constant condition for
// static assertion") when -fsanitize=undefined (or only -fsanitize=null)
// is added. Clang accepts both. The front end gives result 0.
template <class T> inline constexpr char tag = 0;
static_assert(&tag<int> != &tag<long>);
int main() { return 0; }
