#include "wasabi.h"

VALUE rb_mWasabi;

void
Init_wasabi(void)
{
  rb_mWasabi = rb_define_module("Wasabi");
}
