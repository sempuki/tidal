// tidal-rules: OVL-13
// tidal-expect: accept
// tidal-note: The same classes as rewritten-swapped-member-object.cpp with a non-member operator==. Its swapped candidate converts Maker to Right.
struct Left {};
struct Right {};
bool operator==(const Right&, const Left&);
struct Maker { operator Right() const; };
bool r = Left{} == Maker{};
