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

Isolate*
V8_Isolate_New()
{
    Isolate::CreateParams create_params;
    create_params.array_buffer_allocator =
        ArrayBuffer::Allocator::NewDefaultAllocator();
    return Isolate::New(create_params);
}

Local<Context>
V8_Context_New(Isolate* isolate)
{
    Context::New(isolate);
}

void
V8_Context_Enter(Context* context)
{
    context->Enter();
}

void
V8_Context_Exit(Context* context)
{
    context->Exit();
}
}
