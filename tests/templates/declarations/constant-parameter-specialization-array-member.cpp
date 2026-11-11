// tidal-rules: TPL-6 TPL-7
// tidal-expect: run
// tidal-note: Roadmap round 12, programmer reduction class_template_specialization_with_array_member_as_constant_parameter_type.cpp. Span<2> is a specialization of a class template. Its one data member is a public array of int, so Span<2> is a structural type (TPL-6), and TPL-7 does not apply. A plain class with the same member gives the same result.
template <int N> struct Span { int bounds[N]; };
template <Span<2> S> constexpr int width() { return S.bounds[1] - S.bounds[0]; }
static_assert(width<Span<2>{{3, 10}}>() == 7);
int main() { return width<Span<2>{{1, 5}}>() == 4 ? 0 : 1; }
