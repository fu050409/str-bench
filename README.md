# Rust Str Benchmarks

This is a benchmark to compare the performance of `String` and its alternatives in Rust.

## Run the benchmarks

```bash
cargo bench
```

## Results

Benchmarks based on `criterion-rs`.

In this benchmark, we compare the performance of `String`, `ArcStr`, `CompactStr`, `EcowString`, `ImString` and `SmartString`.

### Clone Benchmark Results

Below is a table summarizing the clone benchmark results for different string implementations:

| Type              | Small Clone | Large Clone | Best (✓)  |
| ----------------- | ----------- | ----------- | --------- |
| **String**        | 39.566      | 47.326      |           |
| **CompactString** | **5.725**   | 63.701      | ✓ (small) |
| **SmartString**   | 6.514       | 49.036      |           |
| **EcoString**     | 27.897      | **27.588**  | ✓ (large) |
| **ArcStr**        | 41.876      | 41.328      |           |
| **ImString**      | 27.913      | 27.968      |           |
| **Cow<'a, str>**  | 24.985      | 34.702      |           |

Observations:

- **Best for Small Clones**: **CompactString** (5.725 ns)
- **Best for Large Clones**: **EcoString** (27.588 ns)

CompactString dominates in small string cloning due to its stack allocation strategy, while EcoString performs best for large clones, likely benefiting from efficient reference counting and shared memory handling.

- **CompactString** shows extremely fast clone times for small strings, but its performance for large strings is noticeably slower.
- **SmartString** exhibits similar behavior, with a very low clone time for small strings and moderate clone time for large strings.
- **EcoString**, **ImString**, and **Cow<'a, str>** perform consistently well, with EcoString and ImString being particularly stable across small and large clones.
- Standard **String** and **ArcStr** have higher clone times compared to the optimized alternatives in the small string category, while ArcStr remains competitive for large strings.

### Push Benchmark Results

| Type              | Small Push (ns) | Large Push (µs) | Best (✓)         |
| ----------------- | --------------- | --------------- | ---------------- |
| **String**        | **250.63**      | **7.8998**      | ✓ (small, large) |
| **CompactString** | 371.83          | 15.824          |                  |
| **SmartString**   | 389.45          | 25.142          |                  |
| **EcoString**     | 390.75          | 25.074          |                  |
| **ImString**      | 1,709.9         | 51.173          |                  |

Observations:

- **Best for Small Push**: **String** (250.63 ns)
- **Best for Large Push**: **String** (7.8998 µs)

Rust's native `String` outperforms all alternatives in push operations. This makes sense because `String` is optimized for mutability, while most other alternatives focus on immutability or small-string optimizations that may introduce overhead.

### Iterate Benchmark Results

| Type             | Time (µs) | Best (✓) |
| ---------------- | --------- | -------- |
| **String**       | 4.10      |          |
| CompactString    | 5.27      |          |
| **SmartString**  | **3.19**  | ✓        |
| **EcoString**    | **3.23**  | ✓        |
| **ArcStr**       | **3.26**  | ✓        |
| **Cow<'a, str>** | **3.22**  | ✓        |
| ImString         | 112.93    |          |

Observations:

- **Best performers:** `SmartString`, `EcoString`, `ArcStr`, and `Cow<'a, str>` have nearly identical performance.
- `String` is about **28% slower** than the best performers.
- `CompactString` is even slower than `String`.
- `ImString` is **orders of magnitude slower** and not suitable for iteration-heavy workloads.

### Replace Benchmark Results

| Type          | Small (ns) | Large (µs) | Bad (✗) |
| ------------- | ---------- | ---------- | ------- |
| String        | **150.64** | **14.219** |         |
| CompactString | **148.58** | **14.224** |         |
| SmartString   | **154.26** | **14.158** |         |
| ~~EcoString~~ | 341.15     | 33.262     | ✗       |
| ArcStr        | **149.68** | **14.112** |         |
| ImString      | **149.38** | **14.152** |         |
| ~~Cow~~       | 178.22     | 14.543     | ✗       |

Observations:

- **Best performers:** `CompactString`, `String`, `ArcStr`, `ImString`, and `SmartString` all perform **similarly well**.
- **Worst performer:** `EcoString` is significantly **slower** for replacement operations.
- `Cow<'a, str>` is slightly slower than `String`, but still competitive.

## `to_string` Performance Benchmark Analysis

This benchmark measures the performance of the `to_string()` operation for different string types in Rust. The unit is **nanoseconds (ns)**.

### **Performance Comparison (Sorted by Speed)**

#### **Small Strings (`to_string small`)**

| Type             | Time (ns)  | Best (✓) |
| ---------------- | ---------- | -------- |
| **Cow<'a, str>** | **24.306** | ✓        |
| **ArcStr**       | **25.321** | ✓        |
| ImString         | 46.897     |          |
| EcoString        | 48.066     |          |
| CompactString    | 48.889     |          |
| SmartString      | 51.207     |          |

- **`Cow<'a, str>` and `ArcStr` are the fastest**, with `Cow<'a, str>` **being the absolute best**.
- **ImString > EcoString > CompactString > SmartString**, but the differences are relatively small.

#### **Large Strings (`to_string large`)**

| Type             | Time (ns)  | Best (✓) |
| ---------------- | ---------- | -------- |
| **Cow<'a, str>** | **42.157** | ✓        |
| **ArcStr**       | **42.719** | ✓        |
| CompactString    | 55.565     |          |
| SmartString      | 55.843     |          |
| EcoString        | 59.229     |          |
| ImString         | 59.913     |          |

- **`Cow<'a, str>` and `ArcStr` again dominate**, with `Cow<'a, str>` being the **fastest**.
- `CompactString` and `SmartString` perform similarly but are **~30% slower than `Cow<'a, str>`**.
- `EcoString` and `ImString` are **the slowest**.

### **Recommended Choices**

If your workload **involves frequent `to_string()` calls** and does **not require mutable strings**:

- **Choose `Cow<'a, str>`** (best performance, especially when working with borrowed strings).
- **Alternatively, choose `ArcStr`** (almost as fast as `Cow<'a, str>`, ideal for reference counting).

If `to_string()` is not a bottleneck, other considerations such as **cloning, pushing, or iteration performance** may influence the best choice of string type.

## Detailed Results

See [Benchmark](./BENCHMARK.md).
