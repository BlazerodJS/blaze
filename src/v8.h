#ifndef V8_H
#define V8_H

#include "v8/include/v8.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct blazerod_s Blazerod;

typedef uint32_t blazerod_call_id;

typedef void (*blazerod_call_cb)(blazerod_call_id call_id);

#ifdef __cplusplus
}
#endif

namespace blazerod {

class Isolate {
 public:
  explicit Isolate(blazerod_call_cb cb)
      : isolate_(nullptr), locker_(nullptr), call_cb_(cb) {}

  ~Isolate() { isolate_->Dispose(); }

  static inline Isolate* FromIsolate(v8::Isolate* isolate) {
    return static_cast<Isolate*>(isolate->GetData(0));
  }

  void SetIsolate(v8::Isolate* isolate);

  v8::Isolate* isolate_;
  v8::Locker* locker_;
  v8::Persistent<v8::Context> context_;

  blazerod_call_cb call_cb_;
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
