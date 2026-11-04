// tidal-rules: OVL-42 OVL-37
// tidal-expect: reject
// tidal-error-clang: call to deleted constructor of .Part.
// tidal-error-gcc: use of deleted function .Part::Part\(Part&&\).
// tidal-error-edg: function "Part::Part\(Part &&\)" .* cannot be referenced -- it is a deleted function
// tidal-note: A move constructor declared deleted, and not defaulted, stays a candidate. It wins for the rvalue, and the call is not valid.
struct Part {
  Part() {}
  Part(const Part&) {}
  Part(Part&&) = delete;
};
void use() {
  Part a;
  Part b(static_cast<Part&&>(a));
}
