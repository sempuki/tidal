// tidal-rules: TSP-28 TSP-3
// tidal-expect: run
// tidal-note: An initializer, also an empty braced one, makes the explicit specialization a definition.
template <class T> struct Box { static int count; };
template <> int Box<char>::count = 4;
template <> int Box<long>::count{};
struct Plain { template <class T> static int size; };
template <> int Plain::size<char> = 1;
int main() { return Box<char>::count == 4 && Box<long>::count == 0 && Plain::size<char> == 1 ? 0 : 1; }
