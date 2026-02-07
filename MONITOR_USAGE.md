# Generic Monitor Usage Guide / คู่มือการใช้งาน Generic Monitor

## สรุปภาษาไทย (Thai Summary)

### การใช้งาน Monitor แบบ Generic ใน Library

คู่มือนี้อธิบายวิธีการใช้งาน `Monitor` แบบ Generic ใน Rust library พร้อมทั้ง Zero-Cost Abstraction

#### 1. แนวคิดหลัก

**Generic Monitor Pattern** ช่วยให้คุณสามารถ:
- กำหนด Monitor แบบ Generic ใน Library
- เปิด/ปิด การใช้งาน Monitoring ได้
- เมื่อปิด Monitoring จะไม่มี Overhead เลย (Zero-Cost)

#### 2. Feature Flags สำคัญ

ใน `Cargo.toml`:
```toml
[features]
default = ["b_provider", "monitoring"]
monitoring = []
```

**Build Commands:**
- **เปิด Monitoring:** 
  ```bash
  cargo build --features b_provider,monitoring
  cargo run --example generic_monitor_demo --features b_provider,monitoring
  ```

- **ปิด Monitoring (Zero-Cost):**
  ```bash
  cargo build --no-default-features --features b_provider
  cargo run --example generic_monitor_demo --no-default-features --features b_provider
  ```

#### 3. วิธีการใช้งาน

##### 3.1 ใช้ในฟังก์ชันแบบ Generic

```rust
use rust_challange::{RequestMonitor, create_monitor};

async fn my_function<M: RequestMonitor>(
    data: &str,
    monitor: &M,
) -> Result<String, MyError> {
    monitor.on_start(data);
    
    // ทำงานตามปกติ
    let result = do_something(data).await?;
    
    monitor.on_finish(data, true);
    Ok(result)
}
```

##### 3.2 ใช้ใน Struct แบบ Generic

```rust
use rust_challange::{RequestMonitor, DefaultMonitor, create_monitor};

pub struct MyService<M: RequestMonitor> {
    monitor: M,
    // fields อื่นๆ
}

impl<M: RequestMonitor> MyService<M> {
    pub fn new(monitor: M) -> Self {
        Self { monitor }
    }
    
    async fn process(&self, input: &str) -> Result<String, MyError> {
        self.monitor.on_start(input);
        let result = do_work(input).await?;
        self.monitor.on_finish(input, true);
        Ok(result)
    }
}

// ใช้ DefaultMonitor (จะเป็น NoOpMonitor เมื่อปิด feature)
impl MyService<DefaultMonitor> {
    pub fn with_default_monitor() -> Self {
        Self::new(create_monitor())
    }
}
```

##### 3.3 ใช้ MonitoredOperation Wrapper

```rust
use rust_challange::{DefaultMonitoredOperation, create_monitored};

let monitored = create_monitored();
let result = monitored.execute("data", || async {
    // ทำงานที่ต้องการ monitor
    do_something_async().await
}).await?;
```

#### 4. การสร้าง Custom Monitor

```rust
use rust_challange::RequestMonitor;

#[cfg(feature = "monitoring")]
struct MyCustomMonitor {
    // fields ของคุณ
}

#[cfg(feature = "monitoring")]
impl RequestMonitor for MyCustomMonitor {
    fn on_start(&self, symbol: &str) {
        // ทำสิ่งที่ต้องการเมื่อเริ่ม
        println!("Custom: Starting {}", symbol);
    }
    
    fn on_finish(&self, symbol: &str, success: bool) {
        // ทำสิ่งที่ต้องการเมื่อจบ
        println!("Custom: Finished {} - {}", symbol, success);
    }
}
```

#### 5. ประโยชน์ของ Zero-Cost Abstraction

**เมื่อ Monitoring ถูกปิด:**
- โค้ด monitoring ทั้งหมดจะถูก compile ออก
- `NoOpMonitor` ใช้ `#[inline(always)]` ทำให้ compiler optimize ออกหมด
- **ไม่มี runtime overhead เลย**
- Binary size ลดลง

**เมื่อ Monitoring ถูกเปิด:**
- ใช้ `ConsoleMonitor` หรือ Monitor อื่นๆ
- ทำ logging/tracking ได้ตามต้องการ
- Performance impact น้อยมาก (เพียงแค่ logging)

#### 6. เทคนิคขั้นสูง

##### 6.1 Monitor Factory Pattern

```rust
enum MonitorConfig {
    None,
    Console,
    Detailed,
    Custom,
}

impl MonitorConfig {
    fn create(&self) -> impl RequestMonitor {
        match self {
            #[cfg(feature = "monitoring")]
            MonitorConfig::None => NoOpMonitor,
            #[cfg(feature = "monitoring")]
            MonitorConfig::Console => ConsoleMonitor,
            #[cfg(feature = "monitoring")]
            MonitorConfig::Detailed => DetailedMonitor::default(),
            #[cfg(feature = "monitoring")]
            MonitorConfig::Custom => MyCustomMonitor::new(),
            
            #[cfg(not(feature = "monitoring"))]
            _ => NoOpMonitor,
        }
    }
}
```

##### 6.2 Conditional Compilation ใน Function

```rust
#[cfg(feature = "monitoring")]
fn expensive_monitoring_operation() {
    // โค้ดที่ใช้เวลาและ resource มาก
    // จะถูก compile เมื่อ feature monitoring เปิดเท่านั้น
}

#[cfg(not(feature = "monitoring"))]
fn expensive_monitoring_operation() {
    // โค้ดนี้จะไม่ถูก compile เมื่อปิด monitoring
}
```

---

## English Summary

### Using Generic Monitor in Your Library

This guide explains how to use `Monitor` as a generic type in a Rust library with zero-cost abstraction.

#### 1. Core Concepts

**Generic Monitor Pattern** allows you to:
- Define Monitor as a generic in your library
- Enable/disable monitoring dynamically
- Achieve zero runtime overhead when monitoring is disabled

#### 2. Important Feature Flags

In `Cargo.toml`:
```toml
[features]
default = ["b_provider", "monitoring"]
monitoring = []
```

**Build Commands:**
- **With Monitoring Enabled:**
  ```bash
  cargo build --features b_provider,monitoring
  cargo run --example generic_monitor_demo --features b_provider,monitoring
  ```

- **With Monitoring Disabled (Zero-Cost):**
  ```bash
  cargo build --no-default-features --features b_provider
  cargo run --example generic_monitor_demo --no-default-features --features b_provider
  ```

#### 3. Usage Patterns

##### 3.1 Using in Generic Functions

```rust
use rust_challange::{RequestMonitor, create_monitor};

async fn my_function<M: RequestMonitor>(
    data: &str,
    monitor: &M,
) -> Result<String, MyError> {
    monitor.on_start(data);
    
    // Do your work
    let result = do_something(data).await?;
    
    monitor.on_finish(data, true);
    Ok(result)
}
```

##### 3.2 Using in Generic Structs

```rust
use rust_challange::{RequestMonitor, DefaultMonitor, create_monitor};

pub struct MyService<M: RequestMonitor> {
    monitor: M,
    // other fields
}

impl<M: RequestMonitor> MyService<M> {
    pub fn new(monitor: M) -> Self {
        Self { monitor }
    }
    
    async fn process(&self, input: &str) -> Result<String, MyError> {
        self.monitor.on_start(input);
        let result = do_work(input).await?;
        self.monitor.on_finish(input, true);
        Ok(result)
    }
}

// Use DefaultMonitor (becomes NoOpMonitor when feature is disabled)
impl MyService<DefaultMonitor> {
    pub fn with_default_monitor() -> Self {
        Self::new(create_monitor())
    }
}
```

##### 3.3 Using MonitoredOperation Wrapper

```rust
use rust_challange::{DefaultMonitoredOperation, create_monitored};

let monitored = create_monitored();
let result = monitored.execute("data", || async {
    // Do work that needs monitoring
    do_something_async().await
}).await?;
```

#### 4. Creating Custom Monitors

```rust
use rust_challange::RequestMonitor;

#[cfg(feature = "monitoring")]
struct MyCustomMonitor {
    // your fields
}

#[cfg(feature = "monitoring")]
impl RequestMonitor for MyCustomMonitor {
    fn on_start(&self, symbol: &str) {
        // Do something when starting
        println!("Custom: Starting {}", symbol);
    }
    
    fn on_finish(&self, symbol: &str, success: bool) {
        // Do something when finishing
        println!("Custom: Finished {} - {}", symbol, success);
    }
}
```

#### 5. Benefits of Zero-Cost Abstraction

**When Monitoring is Disabled:**
- All monitoring code is compiled out
- `NoOpMonitor` uses `#[inline(always)]` for compiler optimization
- **Zero runtime overhead**
- Reduced binary size

**When Monitoring is Enabled:**
- Uses `ConsoleMonitor` or other monitors
- Full logging/tracking capabilities
- Minimal performance impact (just logging overhead)

#### 6. Advanced Techniques

##### 6.1 Monitor Factory Pattern

```rust
enum MonitorConfig {
    None,
    Console,
    Detailed,
    Custom,
}

impl MonitorConfig {
    fn create(&self) -> impl RequestMonitor {
        match self {
            #[cfg(feature = "monitoring")]
            MonitorConfig::None => NoOpMonitor,
            #[cfg(feature = "monitoring")]
            MonitorConfig::Console => ConsoleMonitor,
            #[cfg(feature = "monitoring")]
            MonitorConfig::Detailed => DetailedMonitor::default(),
            #[cfg(feature = "monitoring")]
            MonitorConfig::Custom => MyCustomMonitor::new(),
            
            #[cfg(not(feature = "monitoring"))]
            _ => NoOpMonitor,
        }
    }
}
```

##### 6.2 Conditional Compilation in Functions

```rust
#[cfg(feature = "monitoring")]
fn expensive_monitoring_operation() {
    // Expensive monitoring code
    // Only compiled when monitoring feature is enabled
}

#[cfg(not(feature = "monitoring"))]
fn expensive_monitoring_operation() {
    // This code won't be compiled when monitoring is disabled
}
```

---

## Architecture Overview / ภาพรวมสถาปัตยกรรม

### Key Components / ส่วนประกอบหลัก

1. **RequestMonitor Trait** - Core trait for monitoring
2. **NoOpMonitor** - Zero-cost implementation (when feature disabled)
3. **ConsoleMonitor** - Simple console logging (when feature enabled)
4. **DetailedMonitor** - Statistics tracking (when feature enabled)
5. **MonitoredOperation** - Generic wrapper for easy integration
6. **DefaultMonitor** - Type alias that changes based on feature flag

### How Zero-Cost Works / การทำงานแบบ Zero-Cost

```
┌─────────────────────────────────────────────────────────────┐
│                    Your Library Code                          │
│                                                               │
│  async fn fetch<M: RequestMonitor>(monitor: &M) {            │
│      monitor.on_start();     // ← Gets optimized away if      │
│      // ... work ...         //   monitoring is disabled      │
│      monitor.on_finish();    //                              │
│  }                                                            │
└─────────────────────────────────────────────────────────────┘
                          ↓
         ┌────────────────┴────────────────┐
         │                                 │
    [monitoring=true]               [monitoring=false]
         │                                 │
    ┌────▼────┐                      ┌────▼────┐
    │ Console │                      │ NoOp    │
    │ Monitor │                      │ Monitor │
    │         │                      │ (empty) │
    └─────────┘                      └─────────┘
         │                                 │
    Full functionality           Optimized away
    with logging                 by compiler
```

### Type Hierarchy / ลำดับชั้นของ Type

```
RequestMonitor (trait)
├── NoOpMonitor               [always available, zero-cost]
├── ConsoleMonitor            [when monitoring=true]
└── DetailedMonitor           [when monitoring=true]

DefaultMonitor (type alias)
├── NoOpMonitor               [when monitoring=false]
└── ConsoleMonitor            [when monitoring=true]

MonitoredOperation<M>
├── MonitoredOperation<DefaultMonitor>
│   ├── MonitoredOperation<NoOpMonitor>      [zero-cost]
│   └── MonitoredOperation<ConsoleMonitor>
└── MonitoredOperation<DetailedMonitor>
```

---

## Best Practices / แนวทางปฏิบัติที่ดี

### For Library Authors / สำหรับผู้พัฒนา Library

1. **Use Generics for Flexibility**
   - ใช้ Generic parameters สำหรับ Monitor
   - Allows users to provide custom implementations

2. **Provide Sensible Defaults**
   - ให้ DefaultMonitor ที่ทำงานได้ทันที
   - Use `DefaultMonitor` type alias

3. **Document Feature Behavior**
   - อธิบายชัดเจนว่าเมื่อปิด feature จะเกิดอะไร
   - Document what happens when feature is disabled

4. **Keep Monitors Lightweight**
   - ให้ Monitor operations รวดเร็ว
   - Avoid heavy operations in monitoring

### For Library Users / สำหรับผู้ใช้ Library

1. **Choose Right Feature Combination**
   - เลือก feature flags ตามความต้องการ
   - Choose features based on needs

2. **Implement Custom Monitors**
   - สร้าง Monitor ของตัวเองเมื่อจำเป็น
   - Create custom monitors when needed

3. **Test Both Configurations**
   - ทดสอบทั้งเมื่อเปิดและปิด monitoring
   - Test with both enabled and disabled monitoring

4. **Monitor Performance Impact**
   - วัด performance ของ monitoring
   - Measure monitoring performance impact

---

## Examples / ตัวอย่าง

### Example 1: Basic Usage / การใช้งานพื้นฐาน

```rust
use rust_challange::{RequestMonitor, create_monitor};

async fn fetch_data<M: RequestMonitor>(
    url: &str,
    monitor: &M,
) -> Result<String, Error> {
    monitor.on_start(url);
    
    let response = reqwest::get(url).await?.text().await?;
    
    monitor.on_finish(url, true);
    Ok(response)
}

// Usage
let monitor = create_monitor();
let data = fetch_data("https://api.example.com", &monitor).await?;
```

### Example 2: Library Integration / การผสานกับ Library

```rust
// In your library
pub struct MyClient<M: RequestMonitor> {
    base_url: String,
    monitor: M,
}

impl<M: RequestMonitor> MyClient<M> {
    pub async fn get(&self, endpoint: &str) -> Result<String, Error> {
        let url = format!("{}/{}", self.base_url, endpoint);
        self.monitor.on_start(&url);
        
        let result = self.perform_request(&url).await;
        
        self.monitor.on_finish(&url, result.is_ok());
        result
    }
}

// User can provide any monitor
let client = MyClient {
    base_url: "https://api.example.com".to_string(),
    monitor: my_custom_monitor,
};
```

### Example 3: Performance Comparison / การเปรียบเทียบ Performance

```rust
use std::time::Instant;

async fn benchmark_with_monitor<M: RequestMonitor>(
    iterations: usize,
    monitor: &M,
) -> Duration {
    let start = Instant::now();
    
    for i in 0..iterations {
        monitor.on_start(&format!("request_{}", i));
        do_some_work().await;
        monitor.on_finish(&format!("request_{}", i), true);
    }
    
    start.elapsed()
}

// When monitoring=false: minimal overhead
// When monitoring=true: includes logging overhead
```

---

## Troubleshooting / การแก้ปัญหา

### Common Issues / ปัญหาที่พบบ่อย

1. **Feature Not Working**
   - ตรวจสอบว่า feature flag ถูกตั้งค่าถูกต้อง
   - Check if feature flags are set correctly

2. **Monitor Not Optimizing Away**
   - ตรวจสอบว่าใช้ `#[inline(always)]` ใน NoOpMonitor
   - Check for `#[inline(always)]` in NoOpMonitor

3. **Type Mismatches**
   - ตรวจสอบว่า implement RequestMonitor ถูกต้อง
   - Verify correct RequestMonitor implementation

4. **Performance Still Affected**
   - ตรวจสอบว่าไม่ได้ใช้ trait object (`dyn RequestMonitor`)
   - Check for trait object usage

---

## Summary / สรุป

### สรุปภาษาไทย

การใช้งาน Monitor แบบ Generic ใน Rust library ช่วยให้คุณสามารถ:

✅ **กำหนด Monitor แบบ Generic** - ใช้ใน function, struct และ module ต่างๆ  
✅ **Zero-Cost Abstraction** - เมื่อปิด monitoring จะไม่มี overhead เลย  
✅ **Flexible Design** - ผู้ใช้สามารถ implement Monitor ของตัวเองได้  
✅ **Easy Integration** - มี wrapper และ helper ที่ใช้งานง่าย  
✅ **Feature-Based** - ควบคุมผ่าน feature flags ใน Cargo.toml  

**หลักการสำคัญ:**
- ใช้ `#[cfg(feature = "monitoring")]` สำหรับ conditional compilation
- `NoOpMonitor` ใช้ `#[inline(always)]` เพื่อ optimization
- `DefaultMonitor` type alias เปลี่ยนตาม feature flag
- `MonitoredOperation` wrapper ทำให้ใช้งานง่ายขึ้น

### English Summary

Using Generic Monitor in Rust libraries enables you to:

✅ **Define Generic Monitors** - Use in functions, structs, and modules  
✅ **Achieve Zero-Cost Abstraction** - No overhead when monitoring is disabled  
✅ **Flexible Design** - Users can implement custom monitors  
✅ **Easy Integration** - Ready-to-use wrappers and helpers  
✅ **Feature-Based Control** - Manage via Cargo.toml feature flags  

**Key Principles:**
- Use `#[cfg(feature = "monitoring")]` for conditional compilation
- `NoOpMonitor` uses `#[inline(always)]` for optimization
- `DefaultMonitor` type alias changes based on feature flag
- `MonitoredOperation` wrapper simplifies usage

---

## Additional Resources / แหล่งข้อมูลเพิ่มเติม

### Rust Documentation / เอกสาร Rust
- [Generic Traits](https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters)
- [Conditional Compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)
- [Feature Flags](https://doc.rust-lang.org/cargo/reference/features.html)

### Zero-Cost Abstractions / การทำงานแบบ Zero-Cost
- [Rust Zero-Cost Abstractions](https://blog.rust-lang.org/2015/05/11/traits.html)
- [Inline Attributes](https://doc.rust-lang.org/reference/attributes/codegen.html#the-inline-attribute)

---

**Version:** 1.0  
**Last Updated:** 2024  
**License:** Same as parent project  
