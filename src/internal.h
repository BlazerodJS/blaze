#ifndef INTERNAL_H
#define INTERNAL_H

#include "v8/include/v8.h"

#include "v8.h"

namespace blazerod {

class Isolate {
 public:
  explicit Isolate() : isolate_(nullptr), locker_(nullptr) {}

  ~Isolate() { isolate_->Dispose(); }

  static inline Isolate* FromIsolate(v8::Isolate* isolate) {
    return static_cast<Isolate*>(isolate->GetData(0));
  }

  void SetIsolate(v8::Isolate* isolate);

  v8::Isolate* isolate_;
  v8::Locker* locker_;
  v8::Persistent<v8::Context> context_;
};

static inline v8::Local<v8::String> v8_str(const char* x) {
  return v8::String::NewFromUtf8(v8::Isolate::GetCurrent(), x,
                                 v8::NewStringType::kNormal)
      .ToLocalChecked();
}

static inline Isolate* unwrap(Blazerod* d_) {
  return reinterpret_cast<blazerod::Isolate*>(d_);
}

}  // namespace blazerod

extern "C" {
struct blazerod_s {
  blazerod::Isolate isolate;
};
}

#endif
