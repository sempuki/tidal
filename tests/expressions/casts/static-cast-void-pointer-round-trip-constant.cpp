// tidal-rules: CE-4 CAST-1
// tidal-expect: reject
// tidal-error-gcc: cast from .const void\*. is not allowed in a constant expression
// tidal-error-clang: cast from .const void \*. is not allowed in a constant expression
// tidal-error-edg: conversion from .const void \*. to .const Account \*. is invalid
// tidal-note: W20-5, with the writer probe reports/clean-room/roadmap-10/writer-probes/w20-5-void-pointer-round-trip-constant.json. The cast back to the original type is valid at run time (CAST-1), and constant evaluation rejects it (CE-4). Every compiler rejects the static_assert.
struct Account { int balance; };
constexpr int read_back() {
  Account a{40};
  const void* hidden = &a;
  const Account* back = static_cast<const Account*>(hidden);
  return back->balance;
}
static_assert(read_back() == 40);
