// v8 c wrapper
#include "v8wrapper.h"
#include <memory>

using namespace v8;

typedef Local<Value> (*rust_callback)(void*);

extern "C" {

const char*
V8_Version()
{
    const char* version = V8_VERSION_STRING;
    return version;
}

std::unique_ptr<Platform>
V8_Initialize_Platform()
{
    std::unique_ptr<v8::Platform> platform = v8::platform::NewDefaultPlatform();
    v8::V8::InitializePlatform(platform.get());
    V8::Initialize();
    return platform;
}

void
V8_Get_Platform(std::unique_ptr<Platform> platform, Platform** out)
{
    *out = platform.get();
}

/**
 * Associate embedder-specific data with the isolate. |slot| has to be
 * between 0 and GetNumberOfDataSlots() - 1.
 */
void
V8_Isolate_SetData(Isolate* isolate, uint32_t slot, void* data)
{
    isolate->SetData(slot, data);
}

/**
 * Retrieve embedder-specific data from the isolate.
 * Returns NULL if SetData has never been called for the given |slot|.
 */
void*
V8_Isolate_GetData(Isolate* isolate, uint32_t slot)
{
    return isolate->GetData(slot);
}

void
V8_Isolate_With_Locker(Isolate* isolate, rust_callback callback, void* data)
{
    Locker locker(isolate);
    callback(data);
}

Isolate*
V8_FunctionCallbackInfo_GetIsolate(FunctionCallbackInfo<Value>& args)
{
    return args.GetIsolate();
}

Local<Object>
V8_FunctionCallbackInfo_This(FunctionCallbackInfo<Value>& args)
{
    return args.This();
}

int
V8_FunctionCallbackInfo_Length(FunctionCallbackInfo<Value>& args)
{
    return args.Length();
}

Local<Object>
V8_FunctionCallbackInfo_Holder(FunctionCallbackInfo<Value>& args)
{
    return args.Holder();
}

Local<Value>
V8_FunctionCallbackInfo_NewTarget(FunctionCallbackInfo<Value>& args)
{
    return args.NewTarget();
}

bool
V8_FunctionCallbackInfo_IsConstructorCall(FunctionCallbackInfo<Value>& args)
{
    return args.IsConstructCall();
}

Local<Value>
V8_FunctionCallbackInfo_Data(FunctionCallbackInfo<Value>& args)
{
    return args.Data();
}

void
V8_FunctionCallbackInfo_GetReturnValue(FunctionCallbackInfo<Value>& args,
                                       ReturnValue<Value>* out)
{
    *out = args.GetReturnValue();
}

void
V8_ReturnValue_SetLocalValue(ReturnValue<Value>& in, Local<Value> value)
{
    in.Set(value);
}

void
V8_ReturnValue_SetNull(ReturnValue<Value>& in)
{
    in.SetNull();
}

void
V8_ReturnValue_SetUndefined(ReturnValue<Value>& in)
{
    in.SetUndefined();
}

void
V8_ReturnValue_SetBool(ReturnValue<Value>& in, bool value)
{
    in.Set(value);
}

void
V8_ReturnValue_SetDouble(ReturnValue<Value>& in, double value)
{
    in.Set(value);
}

void
V8_ReturnValue_SetInt32(ReturnValue<Value>& in, int32_t value)
{
    in.Set(value);
}

void
V8_ReturnValue_SetUint32(ReturnValue<Value>& in, uint32_t value)
{
    in.Set(value);
}

void
V8_Object_GetInternalField(Local<Object>& obj, int index, Local<Value>* field) {
    *field = obj->GetInternalField(index);
}
}
