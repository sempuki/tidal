// tidal-rules: LAM-22
// tidal-expect: reject
// tidal-error-clang: must match previous return type .void.
// tidal-error-gcc: inconsistent types .void. and .int. deduced
// tidal-error-edg: conflicts with previously deduced type .void.
auto g = [](bool b) {
  if (b) return;
  return 1;
};
