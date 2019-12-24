#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <cstdlib>
#include <cstring>
#include <map>
#include <sstream>
#include <string>

#include "v8/include/libplatform/libplatform.h"
#include "v8/include/v8.h"

#include "v8.h"

namespace blazerod {

void Isolate::SetIsolate(v8::Isolate* isolate) {
  isolate_ = isolate;
  isolate_->SetCaptureStackTraceForUncaughtExceptions(
      true, 10, v8::StackTrace::kDetailed);
  isolate_->SetData(0, this);
}

}  // namespace blazerod

auto defaultAllocator = v8::ArrayBuffer::Allocator::NewDefaultAllocator();
static std::unique_ptr<v8::Platform> defaultPlatform;
// auto defaultPlatform = platform::NewDefaultPlatform();

typedef struct {
  v8::Persistent<v8::Context> ptr;
  v8::Isolate* isolate;

  v8::Persistent<v8::Function> cb;

  std::map<std::string, v8::Eternal<v8::Module>> modules;
  std::map<int, std::map<std::string, v8::Eternal<v8::Module>>> resolved;
} m_engine;

const char* CopyString(std::string str) {
  size_t len = str.length();
  char* mem = (char*)malloc(len + 1);
  memcpy(mem, str.data(), len);
  mem[len] = 0;
  return mem;
}

const char* CopyString(v8::String::Utf8Value& value) {
  if (value.length() == 0) {
    return nullptr;
  }
  return CopyString(*value);
}

void Fprint(FILE* out, const v8::FunctionCallbackInfo<v8::Value>& args) {
  bool first = true;
  for (int i = 0; i < args.Length(); i++) {
    v8::Isolate* isolate = args.GetIsolate();
    v8::HandleScope handle_scope(isolate);
    if (first) {
      first = false;
    } else {
      fprintf(out, " ");
    }
    v8::String::Utf8Value str(isolate, args[i]);
    const char* cstr = CopyString(str);
    fprintf(out, "%s", cstr);
  }
  fprintf(out, "\n");
  fflush(out);
}

void Print(const v8::FunctionCallbackInfo<v8::Value>& args) {
  Fprint(stdout, args);
}

void Log(const v8::FunctionCallbackInfo<v8::Value>& args) {
  Fprint(stderr, args);
}

void Call(const v8::FunctionCallbackInfo<v8::Value>& args) {
  v8::Isolate* isolate = args.GetIsolate();
  blazerod::Isolate* b = blazerod::Isolate::FromIsolate(isolate);
  v8::HandleScope handle_scope(isolate);

  int32_t call_id = 0;
  if (args[0]->IsInt32()) {
    auto context = b->context_.Get(isolate);
    call_id = args[0]->Int32Value(context).FromJust();
  }

  b->call_cb_(call_id);
}

extern "C" {
void blazerod_init() {
  if (defaultPlatform.get() == nullptr) {
    defaultPlatform = v8::platform::NewDefaultPlatform();
    v8::V8::InitializePlatform(defaultPlatform.get());
    v8::V8::Initialize();
  }
}

Blazerod* blazerod_new(blazerod_call_cb call_cb) {
  v8::Isolate::CreateParams params;
  params.array_buffer_allocator = defaultAllocator;

  blazerod::Isolate* b = new blazerod::Isolate(call_cb);
  v8::Isolate* isolate = v8::Isolate::New(params);
  b->SetIsolate(isolate);

  v8::Locker locker(isolate);
  v8::Isolate::Scope isolate_scope(isolate);
  {
    v8::HandleScope handle_scope(isolate);

    v8::Local<v8::ObjectTemplate> global = v8::ObjectTemplate::New(isolate);
    v8::Local<v8::ObjectTemplate> blazerod = v8::ObjectTemplate::New(isolate);

    global->Set(isolate, "Blazerod", blazerod);

    blazerod->Set(isolate, "print", v8::FunctionTemplate::New(isolate, Print));
    blazerod->Set(isolate, "log", v8::FunctionTemplate::New(isolate, Log));
    blazerod->Set(isolate, "call", v8::FunctionTemplate::New(isolate, Call));

    // global :: v8::MaybeLocal<v8::ObjectTemplate>()
    auto context =
        v8::Context::New(isolate, nullptr, global, v8::MaybeLocal<v8::Value>());
    b->context_.Reset(isolate, context);
  }

  return reinterpret_cast<Blazerod*>(b);
}

void blazerod_execute(Blazerod* b_, const char* filename, const char* source) {
  auto* b = blazerod::unwrap(b_);
  auto* isolate = b->isolate_;

  v8::Locker locker(isolate);
  v8::Isolate::Scope isolate_scope(isolate);
  v8::HandleScope handle_scope(isolate);

  auto context = b->context_.Get(isolate);
  assert(!context.IsEmpty());
  v8::Context::Scope context_scope(context);

  auto filename_s = blazerod::v8_str(filename);
  auto source_s = blazerod::v8_str(source);

  v8::TryCatch try_catch(isolate);
  v8::ScriptOrigin origin(filename_s);

  auto script = v8::Script::Compile(context, source_s, &origin);

  if (script.IsEmpty()) {
    printf("script empty\n");
    // TODO: handle errors
    return;
  }

  auto result = script.ToLocalChecked()->Run(context);

  if (result.IsEmpty()) {
    printf("result empty\n");
    // TODO: handle errors
    return;
  }
}

void blazerod_delete(Blazerod* b_) {
  auto b = blazerod::unwrap(b_);
  delete b;
}

const char* blazerod_v8_version() {
  return v8::V8::GetVersion();
}
}
