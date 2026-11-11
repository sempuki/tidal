// From one-line/class_type_template_parameter.cpp and its probe. Pt<1>
// has one public member int[1], so it is a structural type (TPL-7,
// TPL-11). tidal-fe rejects with TPL-7. A class that is not a template
// specialization, with the same member, runs.
template <int N> struct Pt { int x[N]; };
template <Pt<1> P> int get() { return P.x[0]; }
int main() { return get<Pt<1>{{3}}>() - 3; }
