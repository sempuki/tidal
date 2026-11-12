template <class T> inline constexpr char tag = 0;
static_assert(&tag<int> != &tag<long>);
int main() { return &tag<int> == &tag<long>; }
