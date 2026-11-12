template <class T> struct Box { static constexpr int n = 3; };
using B = Box<int>;
int main() { return B::n - 3; }
