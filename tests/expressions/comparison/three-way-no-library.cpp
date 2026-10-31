// tidal-rules: CMP-12
// tidal-expect: reject
// tidal-error-clang: strong_ordering.*was not found
// tidal-error-gcc: strong_ordering. is not a member of .std
// tidal-error-edg: no valid std::strong_ordering type found
auto r = 1 <=> 2;
