// v8 c wrapper
#include "v8wrapper.h"
#include <memory>

using namespace v8;

extern "C" {

std::unique_ptr<Platform>
V8_Initialize_Platform()
{
    std::unique_ptr<v8::Platform> platform = v8::platform::NewDefaultPlatform();
    v8::V8::InitializePlatform(platform.get());
    V8::Initialize();
    return platform;
}

Local<Value>
V8_To_Local_Checked(v8::MaybeLocal<v8::Value> value) {
    return value.ToLocalChecked();
}

void
V8_Template_Set(Local<ObjectTemplate> obj, Local<Name> name, Local<Data> value) {
    obj->Set(name, value);

  // obj->Set(
  //         String::NewFromUtf8(Isolate::GetCurrent(), "foo", NewStringType::kNormal)
  //                 .ToLocalChecked(),
  //         String::NewFromUtf8(Isolate::GetCurrent(), "bar", NewStringType::kNormal)
  //                 .ToLocalChecked());
}
}
