#include "v8/include/v8.h"

#include "internal.h"
#include "v8.h"

namespace blazerod {

void Isolate::SetIsolate(v8::Isolate* isolate) {
  isolate_ = isolate;
  isolate_->SetCaptureStackTraceForUncaughtExceptions(
      true, 10, v8::StackTrace::kDetailed);
  isolate_->SetData(0, this);
}

}  // namespace blazerod
