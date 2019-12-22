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

#include "internal.h"
#include "v8.h"

using namespace v8;

auto defaultAllocator = ArrayBuffer::Allocator::NewDefaultAllocator();
static std::unique_ptr<Platform> defaultPlatform;
// auto defaultPlatform = platform::NewDefaultPlatform();

typedef struct {
  Persistent<Context> ptr;
  Isolate* isolate;

  Persistent<Function> cb;

  std::map<std::string, Eternal<Module>> modules;
  std::map<int, std::map<std::string, Eternal<Module>>> resolved;
} m_engine;

const char* CopyString(std::string str) {
  size_t len = str.length();
  char* mem = (char*)malloc(len + 1);
  memcpy(mem, str.data(), len);
  mem[len] = 0;
  return mem;
}

const char* CopyString(String::Utf8Value& value) {
  if (value.length() == 0) {
    return nullptr;
  }
  return CopyString(*value);
}

void Fprint(FILE* out, const FunctionCallbackInfo<Value>& args) {
  bool first = true;
  for (int i = 0; i < args.Length(); i++) {
    Isolate* isolate = args.GetIsolate();
    HandleScope handle_scope(isolate);
    if (first) {
      first = false;
    } else {
      fprintf(out, " ");
    }
    String::Utf8Value str(isolate, args[i]);
    const char* cstr = CopyString(str);
    fprintf(out, "%s", cstr);
  }
  fprintf(out, "\n");
  fflush(out);
}

void Print(const FunctionCallbackInfo<Value>& args) {
  Fprint(stdout, args);
}

void Log(const FunctionCallbackInfo<Value>& args) {
  Fprint(stderr, args);
}

extern "C" {
void blazerod_init() {
  if (defaultPlatform.get() == nullptr) {
    defaultPlatform = platform::NewDefaultPlatform();
    V8::InitializePlatform(defaultPlatform.get());
    V8::Initialize();
  }
}

Blazerod* blazerod_new() {
  Isolate::CreateParams params;
  params.array_buffer_allocator = defaultAllocator;

  blazerod::Isolate* b = new blazerod::Isolate();
  v8::Isolate* isolate = v8::Isolate::New(params);
  b->SetIsolate(isolate);

  v8::Locker locker(isolate);
  v8::Isolate::Scope isolate_scope(isolate);
  {
    v8::HandleScope handle_scope(isolate);

    Local<ObjectTemplate> global = ObjectTemplate::New(isolate);
    Local<ObjectTemplate> v8engine = ObjectTemplate::New(isolate);

    global->Set(isolate, "V8Engine", v8engine);

    v8engine->Set(isolate, "print", FunctionTemplate::New(isolate, Print));
    v8engine->Set(isolate, "log", FunctionTemplate::New(isolate, Log));
    // v8engine->Set(isolate, "cb", FunctionTemplate::New(isolate, cb));

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
  return V8::GetVersion();
}
}
