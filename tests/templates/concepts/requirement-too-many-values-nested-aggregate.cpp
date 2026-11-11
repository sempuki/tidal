// tidal-rules: CPT-26 LIST-12 LIST-4
// tidal-expect: run
// tidal-note: Roadmap round 14, programmer reduction requires_too_many_braced_initializers_nested_aggregate.cpp and probe requires_too_many_braced_initializers_array_member.cpp (from the tries of fields.cpp). The last member of each class is an aggregate or an array of one element. Two values fill the first member and, with the inner braces omitted, the last member (LIST-4). A third value has no member (LIST-12), so the requirement with three values does not hold (CPT-26), and the program stays valid.
struct Inner { int r; };
struct WithAggregate { short x; Inner in; };
struct WithArray { short x; int a[1]; };
template <class T> concept takes_two = requires { T{1, 2}; };
template <class T> concept takes_three = requires { T{1, 2, 3}; };
static_assert(takes_two<WithAggregate> && !takes_three<WithAggregate>);
static_assert(takes_two<WithArray> && !takes_three<WithArray>);
int main() {
  bool fits = takes_two<WithAggregate> && takes_two<WithArray>;
  bool too_many = takes_three<WithAggregate> || takes_three<WithArray>;
  return fits && !too_many ? 0 : 1;
}
