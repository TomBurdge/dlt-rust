# Archive Note: This Repository was Archived on 29/06/2025
This was a fun experiment, which helped me to learn and understand rust inter-op with python.

### The Facts
* Re-factored rust speedup was marginal in terms of speedup over dlt's already very well optimised code.
* Reason: API Ingestion's bottleneck is I/O, and the rust code is not async.
* Re-factoring the rust code would not too hard (although it would have been easier to write it async first) and I have [refactored a crate from sync to async before](https://github.com/maxwellflitton/dockpack/pull/11).

### Project learning outcomes:
1. Even if the rust code were async, this still might not have led to faster code:
  * [Async PyO3](https://github.com/PyO3/pyo3-async-runtimes) is not pro-actively maintained, and compounds the existing issues with pyo3 (magic macros which are really hard to debug when they fail, and can lead to super long compile times).
  * Even async pyo3/rust inter-op is limited by the (infamous) python gil.
  * An alternative to async pyo3 be to make the inter-opping functions create and complete their own runtimes, which could also have performance implications.
  * Some of dlt's core code architecture (it's either concurrent or async for multiple sources, but definitely async within a source for remote APIs) could compound this gil limitation by limiting the rust asynchronicity. What is great for performant python won't necessarily be great for rust/python inter-op that takes advantage of both's strengths.
* For the above reasons, performance improvement could therefore only reached with a decent amount of iteration and some proper code profiling; beyond the scope of an exploratory project.

2. If rust could lead to a faster ingestion pipeline, it would also be a different style of pipeline from an ingestion pipeline.
* In particular, rust's approach to serialization/deserialization (not necessarily correct but how I think of it: inputs to rust types and rust types to outputs) is very strict in the most commonly used crate; `serde`. Whereas DLT prides itself on flexibility for schema evolution (which is frustratingly unavoidable in ingestion/data engineering), rust's strictness does not lend itself well to this.
* Some macro magic could get around rust's rigidity, but there are tradeoffs there too (maintainability/accessibility/readability/potential for bug frequency).

3. DLT's integration with arrow is lovely. It means that the core python DLT library is already about as performant as one could hope for in pure python.

Thankfully, you don't always need to produce great, working software to learn something. :)

# dlt-rust
Ingest data from an API with Data Load Tool (DLT) via a rust pyo3 plugin.

# 1 Concepts
[Data Load Tool](https://github.com/dlt-hub/dlt) (DLT) is an open source python library, which simplified data loading/ingestion for many sources. DLT is almost entirely written in python. The code is well written, with built in and easy to use implementations for [asynchronous and parallel execution](https://dlthub.com/docs/reference/performance/#parallelism-within-a-pipeline).<br><br>

[Pyo3](https://github.com/PyO3/pyo3) is an open source crate which provides python bindings for rust.<br><br>


# 2 Motivation
DLT's python-native parallelism/concurrency will still be limited by Global Interpreter Lock (the GIL).<br>
Defining the GIL in detail is beyond the scope of this README, but in short it is "a mutex (or a lock) that allows only one thread to hold the control of the Python interpreter." [*](https://realpython.com/python-gil/)<br><br>
The GIL is part of what makes python an accessible language, where users do not have to consider components of memory management that exist in other languages.
With this comes trade-offs; the GIL limits [significantly limits the the performance gains](https://realpython.com/python-gil/#the-impact-on-multi-threaded-python-programs) that can result from multi-threaded python-native code.<br><br>

Although python is the industry-standard language of choice for many data engineering use cases, it often provides best performance when used as a wrapper for faster languages.<br>Rust is a system levels language, which gives the user significantly greater control over memory management.<br>
Partially as a result of this greater control: "fearless concurrency"[*](https://doc.rust-lang.org/book/ch16-00-concurrency.html) and [user-brought async runtimes](https://doc.rust-lang.org/book/ch17-00-async-await.html).

Components of ingestion benefit from asynchronicity: a common data ingestion use-case involves multiple calls to a single API endpoint with different headers.<br>
Making these calls at the same time/within quick succession, while cohering to an endpoint's rate limits, offers potential performance benefits.<br><br>

## 2.1 Additional Benefits
[Data contracts](https://learn.microsoft.com/en-us/azure/cloud-adoption-framework/scenarios/cloud-scale-analytics/architectures/data-contracts) is a popular concept within data engineering: variously implicit/explicit commitments/agreements between a data consumer/sender for data availability and correctness.<br><br>

Detailed comments on data contracts are beyond the scope of this README, but the hype around the idea has struggled to gain the same traction in industry practice.<br><br>

Nonetheless, the data ingestor-supplier relationship does involve implicit roles and responsibilities, which are weighted on the data supplier. For example, a data ingestor can reasonably expect that a data supplier provide well-structured data whose form does not change without due warning.<br><br>

DLT offers what it calls ["schema and data contracts"](https://dlthub.com/docs/general-usage/schema-contracts) for schema validation and custom DQ checks. Impressively, [partial schema evolution](https://dlthub.com/docs/general-usage/schema-evolution) is also supported.<br><br>

These forms of checks are opt-in, via [Pydantic](https://github.com/pydantic/pydantic) or custom validation.<br>Particularly with API ingestion, one should not expect the schema to change without warning. Strict-by-design, with rust's excellent serialization/de-serialization support through [serde](https://docs.rs/serde/latest/serde/), can be a worthy approach.

# 3 Rust Plugin
DLT offers plugins; custom ingestion [sources](https://dlthub.com/docs/dlt-ecosystem/verified-sources/) and [destinations](https://dlthub.com/docs/dlt-ecosystem/destinations/) which extend or replace DLT's existing in-built features.<br><br>
Through pyo3, I have written a custom source in rust. This rust is a [re-implementation](http://rewriteitinrust.studiosi.es/img/rewrite_in_rust_1.jpg) of the [chess.com verified DLT source](https://dlthub.com/docs/dlt-ecosystem/verified-sources/chess).<br><br>
Through this, we can compare performance and features.

## 3.1 Arrow: (nearly) Zero-Copy Rust-Python Data Transfer
[DLT offers support for Arrow to sink data to its location and metadata via Apache Arrow](https://dlthub.com/blog/how-dlt-uses-apache-arrow). [Apache Arrow](https://arrow.apache.org/) is an in-memory format which allows for fast serialization to parquet files, and fast memory transfer between languages in a program with zero-copy.<br><br>
Pyo3 plugins can be limited in their performance through conversion between rust and python types. [Pyarrow](https://pypi.org/project/pyarrow/) is an implementation of arrow for python. [arrow-rs supports zero-copy conversion to pyarrow objects](https://docs.rs/arrow/latest/arrow/pyarrow/index.html), which can then be processed by python. DLT can then process the source's ingestion data, to be then processed before sinking to the destination, without a conversion process that could affect performance.
