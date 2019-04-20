#![allow(dead_code)]
use std::mem;

use crate::v8::{
    raw,
    raw::internal::{
        Address,
        Internals_kUndefinedValueRootIndex,
    },
    prelude::*,
    Rooted,
    Isolated,
    Local,
    Value,
    Primitive,
    Context,
    HandleScope,
    PromiseHook,
    utils,
};

pub use crate::v8::{
    raw::{
        EmbedderHeapTracer,
        MicrotasksPolicy,
        MicrotasksPolicy_kExplicit,
        MicrotasksPolicy_kScoped,
        MicrotasksPolicy_kAuto,

        MicrotasksCompletedCallback,
        CounterLookupCallback,
        Isolate_UseCounterCallback,
        Isolate_UseCounterFeature,
        Isolate_UseCounterFeature_kUseAsm,
        Isolate_UseCounterFeature_kBreakIterator,
        Isolate_UseCounterFeature_kLegacyConst,
        Isolate_UseCounterFeature_kMarkDequeOverflow,
        Isolate_UseCounterFeature_kStoreBufferOverflow,
        Isolate_UseCounterFeature_kSlotsBufferOverflow,
        Isolate_UseCounterFeature_kObjectObserve,
        Isolate_UseCounterFeature_kForcedGC,
        Isolate_UseCounterFeature_kSloppyMode,
        Isolate_UseCounterFeature_kStrictMode,
        Isolate_UseCounterFeature_kStrongMode,
        Isolate_UseCounterFeature_kRegExpPrototypeStickyGetter,
        Isolate_UseCounterFeature_kRegExpPrototypeToString,
        Isolate_UseCounterFeature_kRegExpPrototypeUnicodeGetter,
        Isolate_UseCounterFeature_kIntlV8Parse,
        Isolate_UseCounterFeature_kIntlPattern,
        Isolate_UseCounterFeature_kIntlResolved,
        Isolate_UseCounterFeature_kPromiseChain,
        Isolate_UseCounterFeature_kPromiseAccept,
        Isolate_UseCounterFeature_kHtmlCommentInExternalScript,
        Isolate_UseCounterFeature_kHtmlComment,
        Isolate_UseCounterFeature_kSloppyModeBlockScopedFunctionRedefinition,
        Isolate_UseCounterFeature_kForInInitializer,
        Isolate_UseCounterFeature_kArrayProtectorDirtied,
        Isolate_UseCounterFeature_kArraySpeciesModified,
        Isolate_UseCounterFeature_kArrayPrototypeConstructorModified,
        Isolate_UseCounterFeature_kArrayInstanceProtoModified,
        Isolate_UseCounterFeature_kArrayInstanceConstructorModified,
        Isolate_UseCounterFeature_kLegacyFunctionDeclaration,
        Isolate_UseCounterFeature_kRegExpPrototypeSourceGetter,
        Isolate_UseCounterFeature_kRegExpPrototypeOldFlagGetter,
        Isolate_UseCounterFeature_kDecimalWithLeadingZeroInStrictMode,
        Isolate_UseCounterFeature_kLegacyDateParser,
        Isolate_UseCounterFeature_kDefineGetterOrSetterWouldThrow,
        Isolate_UseCounterFeature_kFunctionConstructorReturnedUndefined,
        Isolate_UseCounterFeature_kAssigmentExpressionLHSIsCallInSloppy,
        Isolate_UseCounterFeature_kAssigmentExpressionLHSIsCallInStrict,
        Isolate_UseCounterFeature_kPromiseConstructorReturnedUndefined,
        Isolate_UseCounterFeature_kConstructorNonUndefinedPrimitiveReturn,
        Isolate_UseCounterFeature_kLabeledExpressionStatement,
    },
};

mod tasks;
pub use tasks::*;

extern {
    fn V8_Isolate_With_Locker(
        isolate: *const raw::Isolate,
        callback: extern fn(data: *mut std::ffi::c_void),
        data: *mut std::ffi::c_void);
}

extern "C" {
    fn V8_Isolate_SetData(isolate: *mut raw::Isolate, slot: u32, data: *mut std::ffi::c_void);
    fn V8_Isolate_GetData(isolate: *mut raw::Isolate, slot: u32) -> *mut std::ffi::c_void;
}

/// default slot for internal usage.
pub const ISOLATE_DATA_SLOT: u32 = 0;
/// internal Isolate data
pub struct IsolateData {
    pub count: usize,
}

/// trampoline function for:
///     typedef (*wrapper)(void* data)
extern fn callback_data_wrapper(data: *mut std::ffi::c_void) {
    unsafe {
        let closure: &mut Box<FnMut()> = mem::transmute(data);
        closure()
    }
}

/// trampoline function for:
///     typedef (*wrapper)(Isolate* isolate, void* data)
extern fn callback_isolate_wrapper(isolate: *mut raw::Isolate, data: *mut std::ffi::c_void) {
    unsafe {
        let closure: &mut Box<FnMut(&mut raw::Isolate)> = mem::transmute(data);
        closure(isolate.as_mut().unwrap())
    }
}

/// trampoline function for:
///     typedef int* (*wrapper)(char* name)
extern fn callback_counter_wrapper(name: *const std::os::raw::c_char) -> *mut std::os::raw::c_int {
    unimplemented!()
}

#[repr(C)]
pub struct Isolate(pub *mut raw::Isolate);

impl Isolate {
    pub fn New() -> Self {
        let isolate = unsafe {
            let mut create_params: raw::Isolate_CreateParams = mem::zeroed();
            create_params.array_buffer_allocator = raw::ArrayBuffer_Allocator::NewDefaultAllocator();
            raw::Isolate::New(&create_params)
        };
        if isolate.is_null() {
            panic!("create isolate failed");
        } else {
            unsafe {
                let init_data_ptr =
                    Box::into_raw(Box::new(
                            IsolateData {
                                count: 1,
                            }));
                V8_Isolate_SetData(isolate, ISOLATE_DATA_SLOT, init_data_ptr as *mut std::ffi::c_void);
            }
            Self(isolate)
        }
    }

    /// Returns the entered isolate for the current thread or NULL in
    /// case there is no current isolate.
    ///
    /// If it is not convenient to pass the reference of Isolate, you can use
    /// this static function.
    #[inline]
    pub fn Current() -> Self {
        unsafe {
            let isolate: Self = mem::transmute(raw::Isolate_GetCurrent());
            let ref mut data = isolate.get_data::<IsolateData>(ISOLATE_DATA_SLOT);
            data.count += 1;
            isolate
        }
    }

    /// Get the reference of raw Isolate.
    #[inline]
    pub fn current(&self) -> &mut raw::Isolate {
        unsafe {
            self.0.as_mut().unwrap()
        }
    }

    /// Returns true if this isolate has a current context.
    #[inline]
    pub fn in_context(&mut self) -> bool {
        unsafe {
            self.InContext()
        }
    }

    /// Returns the context of the currently running JavaScript, or the context
    /// on the top of the stack if no JavaScript is running.
    #[inline]
    pub fn get_current_context(&mut self) -> Local<Context> {
        unsafe {
            self.GetCurrentContext()
        }
    }

    /// Returned the entered context.
    #[inline]
    pub fn get_entered_context(&mut self) -> Local<Context> {
        unsafe {
            self.GetEnteredContext()
        }
    }

    /// Returns either the last context entered through V8's C++ API, or the
    /// context of the currently running microtask while processing microtasks.
    /// If a context is entered while executing a microtask, that context is
    /// returned.
    #[inline]
    pub fn get_entered_or_microtask_context(&mut self) -> Local<Context> {
        unsafe {
            self.GetEnteredOrMicrotaskContext()
        }
    }

    /// Returns the Context that corresponds to the Incumbent realm in HTML spec.
    /// https://html.spec.whatwg.org/multipgage/webappapis.html#incumbent
    #[inline]
    pub fn get_incumbent_context(&mut self) -> Local<Context> {
        unsafe {
            self.GetIncumbentContext()
        }
    }

    /// Schedules an exception to be thrown when returning to JavaScript. When an
    /// exception has been scheduled it is illegal to invoke any JavaScript
    /// operation; the caller must return immediately and only after the exception
    /// has been handled does it become legal to invoke JavaScript operation.
    #[inline]
    pub fn throw_exception(&mut self, exception: Local<Value>) -> Local<Value> {
        unsafe {
            self.ThrowException(exception)
        }
    }

    /// Sets the embedder heap tracer for the isolate.
    #[inline]
    pub fn set_embedder_heap_tracer(&mut self, tracer: *mut EmbedderHeapTracer) {
        unsafe {
            self.SetEmbedderHeapTracer(tracer)
        }
    }

    /// Gets the embedder heap tracer for the isolate.
    #[inline]
    pub fn get_embedder_heap_tracer(&mut self) -> *mut EmbedderHeapTracer {
        unsafe {
            self.GetEmbedderHeapTracer()
        }
    }

    /// Get data ptr in slot.
    #[inline]
    pub fn get_data_ptr<T>(&self, slot: u32) -> *mut T {
        unsafe {
            V8_Isolate_GetData(self.0, slot) as *mut T
        }
    }

    /// Get data in slot.
    #[inline]
    pub fn get_data<T>(&self, slot: u32) -> &mut T {
        unsafe {
            let ptr = self.get_data_ptr(slot) as *mut T;
            match ptr.as_mut() {
                Some(ptr) => ptr,
                None => panic!(format!("Isolate::GetData with slot: {} got nothing.", slot)),
            }
        }
    }

    /// Get Default Internal Data
    #[inline]
    pub fn get_0(&self) -> &mut IsolateData {
        self.get_data::<IsolateData>(0)
    }

    /// Set data to slot.
    #[inline]
    pub fn set_data<T>(&mut self, slot: u32, data: T) {
        unsafe {
            let data_ptr = Box::into_raw(Box::new(data));
            V8_Isolate_SetData(self.0, slot, data_ptr as *mut std::ffi::c_void);
        }
    }

    /// Drop data in slot.
    #[inline]
    pub fn drop_data<T>(&mut self, slot: u32) {
        unsafe {
            drop(Box::from_raw(self.get_data_ptr::<T>(slot)))
        }
    }

    /// Gets Isolate Reference Count
    #[inline]
    pub fn reference_count(&mut self) -> usize {
        self.get_data::<IsolateData>(ISOLATE_DATA_SLOT).count
    }

    /// Helper function to run your function.
    /// Providing the current context, returns V8Result<T>
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate rvb;
    /// use rvb::v8::{
    ///     prelude::*,
    ///     Platform,
    ///     Isolate,
    /// };
    /// fn main () {
    ///     let _platform = Platform::New();
    ///     let mut isolate = Isolate::New();
    ///
    ///     isolate.exec(|ctx| {
    ///         Ok(())
    ///     }).unwrap();
    /// }
    /// ```
    #[inline]
    pub fn exec<U, F>(&mut self, run: F) -> V8Result<U>
        where F: FnOnce(Local<Context>) -> V8Result<U>
        {
            self.enter();
            let _handle_scole = HandleScope::New();
            let mut context = V8Context::Default();
            context.enter();

            let result = run(context)?;

            context.exit();
            self.exit();

            Ok(result)
        }

    /// Tells the VM whether the embedder is idle or not.
    #[inline]
    pub fn set_idle(&mut self, is_idle: bool) {
        unsafe {
            self.SetIdle(is_idle)
        }
    }

    /// Check if V8 is dead and thereforce unusable. This is the case after
    /// fatal errors such as out-of-meomory situations.
    ///
    /// # Usage
    ///
    /// ```rust
    /// use rvb::v8::{Platform, Isolate};
    /// let _platform = Platform::New();
    /// let mut isolate = Isolate::New();
    /// assert!(!isolate.is_dead());
    /// ```
    #[inline]
    pub fn is_dead(&mut self) -> bool {
        unsafe {
            self.IsDead()
        }
    }

    /// Check if this isolate is in use.
    /// True if at least one thread Enter'ed this isolate.
    #[inline]
    pub fn is_in_use(&mut self) -> bool {
        unsafe {
            self.IsInUse()
        }
    }

    /// Returns the ArrayBuffer::Allocator used in this isolate.
    #[inline]
    pub fn get_array_buffer_allocator(&mut self) -> *mut raw::ArrayBuffer_Allocator {
        unsafe {
            self.GetArrayBufferAllocator()
        }
    }

    /// Disposes the isolate. The isolate must not be entered by any
    /// thread to be disposable.
    #[inline]
    pub fn dispose(&mut self) {
        unsafe {
            self.Dispose()
        }
    }

    // Internals::GetRoot implementation
    #[inline]
    pub fn get_root(&self, index: i32) -> *mut Address {
        utils::get_root(self.current(), index)
    }

    /// v8::Undefined(Isolate* isolate)
    #[inline]
    pub fn undefined(&self) -> Local<Primitive> {
        let slot = self.get_root(Internals_kUndefinedValueRootIndex);
        unsafe {
            mem::transmute(slot)
        }
    }

    /// Isolate Run Within A V8::Locker
    #[inline]
    pub fn with_locker<F>(&mut self, mut run: F)
        where F: FnMut(V8Context)
        {
            let mut cloned = self.clone();
            let callback: Box<Box<FnMut()>>
                = Box::new(Box::new(|| {
                    cloned.enter();
                    let _handle_scole = HandleScope::New();
                    let mut context = V8Context::Default();
                    context.enter();
                    run(context);
                    context.exit();
                    cloned.exit();
                }));

            unsafe {
                V8_Isolate_With_Locker(
                    self.0,
                    callback_data_wrapper,
                    Box::into_raw(callback) as *mut std::ffi::c_void)
            }
        }

    /// Set the PromiseHook callback for various promise lifecycle
    /// events.
    #[inline]
    pub fn set_promise_hook(&mut self, hook: PromiseHook) {
        unsafe {
            self.SetPromiseHook(hook)
        }
    }

    /// Runs the default MicrotaskQueue until it gets empty.
    /// Any exceptions thrown by microtask callbacks are awallowed.
    #[inline]
    pub fn run_microtasks(&mut self) {
        unsafe {
            self.RunMicrotasks()
        }
    }

    /// Enqueues the callback to the default MicrotaskQueue
    #[inline]
    pub fn enqueue_microtask(&mut self, microtask: V8Function) {
        unsafe {
            self.EnqueueMicrotask(microtask)
        }
    }

    /// Enqueues a rust closure to the default MicrotaskQueue
    ///
    /// # Usage
    ///
    /// ```rust
    /// use rvb::v8::{Platform, Isolate};
    /// let _platform = Platform::New();
    /// let mut isolate = Isolate::New();
    /// isolate.enqueue_closure(|_isolate| {
    ///     println!("Hello from enqueued closure. #1");
    /// });
    /// isolate.enqueue_closure(|_isolate| {
    ///     println!("Hello from enqueued closure. #2");
    /// });
    /// isolate.run_microtasks();
    /// ```
    #[inline]
    pub fn enqueue_closure<F>(&mut self, mut closure: F)
        where F: FnMut()
        {
            let callback: Box<Box<FnMut()>>
                = Box::new(Box::new(|| {
                    closure()
                }));
            unsafe {
                self.EnqueueMicrotask1(
                    Some(callback_data_wrapper),
                    Box::into_raw(callback) as *mut std::ffi::c_void)
            }
        }

    /// Controls how Microtasks are invoked. See MicrotasksPolicy for details.
    #[inline]
    pub fn set_microtasks_policy(&mut self, policy: MicrotasksPolicy) {
        unsafe {
            self.SetMicrotasksPolicy(policy)
        }
    }

    /// Returns the policy controlling how Microtasks are invoked.
    /// Default: MicrotasksPolicy_kAuto.
    ///
    /// # Usage
    ///
    /// ```rust
    /// use rvb::v8::{Platform, Isolate, MicrotasksPolicy_kAuto};
    /// let _platform = Platform::New();
    /// let mut isolate = Isolate::New();
    /// assert_eq!(MicrotasksPolicy_kAuto, isolate.get_microtasks_policy());
    /// ```
    #[inline]
    pub fn get_microtasks_policy(&mut self) -> MicrotasksPolicy {
        unsafe {
            self.GetMicrotasksPolicy()
        }
    }

    #[inline]
    pub fn add_microtasks_completed_callback(&mut self, callback: MicrotasksCompletedCallback) {
        unsafe {
            self.AddMicrotasksCompletedCallback(callback)
        }
    }

    /// Hook when microtasks completed
    ///
    /// See [examples/tasks.rs]
    #[inline]
    pub fn add_microtasks_completed_closure<F>(&mut self, closure: F)
        where F: FnMut(&mut raw::Isolate)
        {
            let callback: Box<Box<FnMut(&mut raw::Isolate)>>
                = Box::new(Box::new(closure));
            unsafe {
                self.AddMicrotasksCompletedCallback1(
                    Some(callback_isolate_wrapper),
                    Box::into_raw(callback) as *mut std::ffi::c_void)
            }
        }

    #[inline]
    pub fn remove_microtasks_completed_callback(&mut self, callback: MicrotasksCompletedCallback) {
        unsafe {
            self.RemoveMicrotasksCompletedCallback(callback)
        }
    }

    #[inline]
    pub fn remove_microtasks_completed_closure<F>(&mut self, closure: F)
        where F: FnMut(&mut raw::Isolate)
        {
            let callback: Box<Box<FnMut(&mut raw::Isolate)>>
                = Box::new(Box::new(closure));
            unsafe {
                self.RemoveMicrotasksCompletedCallback1(
                    Some(callback_isolate_wrapper),
                    Box::into_raw(callback) as *mut std::ffi::c_void)
            }
        }

    #[inline]
    pub fn set_use_counter_callback(&mut self, callback: Isolate_UseCounterCallback) {
        unsafe {
            self.SetUseCounterCallback(callback)
        }
    }

    /// Enables the host application to provide a mechanism for recording
    /// statistics counters.
    #[inline]
    pub fn set_counter_function(&mut self, callback: CounterLookupCallback) {
        unsafe {
            self.SetCounterFunction(callback)
        }
    }

    #[inline]
    pub fn set_counter_name<N: ToString>(&mut self, name: N) {
        unimplemented!()
    }
}

deref_mut!(Isolate);

impl Rooted for Isolate {
    fn allocate() -> Self {
        Isolate::New()
    }

    fn enter(&mut self) {
        unsafe {
            self.Enter()
        }
    }

    fn exit(&mut self) {
        unsafe {
            self.Exit()
        }
    }
}

/// Clone an Isolate
impl Clone for Isolate {
    fn clone(&self) -> Isolate {
        self.get_data::<IsolateData>(ISOLATE_DATA_SLOT).count += 1;
        Isolate(self.0)
    }
}

/// Drop an Isolate
impl Drop for Isolate {
    fn drop(&mut self) {
        let ref mut data = self.get_0();
        data.count -= 1;

        if data.count == 0 {
            self.dispose();
        }
    }
}
