#include "mode.h"
int next_code(Mode m) {
  switch (m) {
  case Mode::idle: return static_cast<int>(Mode::busy);
  case Mode::busy: return static_cast<int>(Mode::done);
  case Mode::done: return static_cast<int>(Mode::idle);
  }
  return -1;
}
