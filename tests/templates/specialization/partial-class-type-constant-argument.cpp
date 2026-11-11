// tidal-rules: TSP-14 TARG-19
// tidal-expect: run
// tidal-note: Roadmap round 13, programmer reduction partial_specialization_class_type_constant_argument.cpp (from the tries of units.cpp). The partial specialization fixes the first argument to a value of class type. A list whose first argument is another value of that class is a different list (TARG-19, by the class row of TARG-P4), so only the primary template matches it. The second parameter has a default, which the given list includes.
struct Dim {
  int length = 0;
  int time = 0;
  constexpr bool operator==(const Dim&) const = default;
};
template <Dim V, class T = int> struct Quantity {};
template <class X> struct is_speed { static constexpr bool value = false; };
template <class T> struct is_speed<Quantity<Dim{1, -1}, T>> { static constexpr bool value = true; };
int main() {
  int bad = 0;
  if (!is_speed<Quantity<Dim{1, -1}>>::value) bad += 1;
  if (!is_speed<Quantity<Dim{1, -1}, long>>::value) bad += 2;
  if (is_speed<Quantity<Dim{1, 0}>>::value) bad += 4;
  if (is_speed<Quantity<Dim{0, -1}, long>>::value) bad += 8;
  return bad;
}
