# dlt-rust
Ingest data from an API with Data Load Tool (DLT) via a rust pyo3 plugin.

# 1 Ingestion
Data ingestion the component of data engineering which involves receiving data from an outside source, and loading the data within one's own environment.<br><br>
Common use cases for ingestion, in my experience in an enterprise setting, are threefold:
1. Ingestion of data from a data suppplier outside one's own organisation.
2. Ingestion of data from an upstream team or environment in the data lifecycle.
3. Migration of data between platforms or, less commonly, environments (dev/prod).
<br><br>
# 2 Sources of Complexity
This definition of ingestion allows us to consider three components of complexity in ingestion:
## 2.1 **Complexity of sources:**
One's environment (ingestion destination) is likely to be significantly different from diverse ingestion sources.<br>Upstream data could be in diverse forms, including but not limited to: APIs, diverse flat files (excel, parquet, .wav), databases, and message streams.<br>These data sources can all exist with complex varieties of latency and schemas.
## 2.2 **Complexity of destinations**
One's own file destination should be more consistent; in a data engineering team it is industry best practice to store data in an open table format (delta/iceberg) in cloud file storage.<br>Data catalogs, which are effectively the previous pattern with more built-in metadata capabilities, are becoming more common but not universal.<br>
Nonetheless, different teams can work with: 
* Different clouds
* Different networking security
* Different data models.
* Different standards of code, or inherited legacy cloud/code components.

Through diversity of systems, there remains complexity.
## 2.3 **Complexity of teams**<br>

Technical components are important, but team structures are often the most important component of complexity. In any organisation of reasonable size and geographic dispersal, ingestion between sources and destinations remains increasingly complex. Team/communication interfaces that result in complexity, for ingestion, includes: 
1. Communication surrounding source/destination authentication.
2. Communication and dependencies on source availability.
3. Complexity of verifying source data quality and communication for resolution.
<br>
---

Data ingestion can sound simple: move data from one place to another. However, the above components result in complex patterns; include, with this, a high number of diverse data sources, and data ingestion becomes a hard problem in need of common patterns for simplification.
# 3 Routes to Simplicity

## Build it
"Frameworks"
* YAML Engineering: 
* No Code:
## Buy it
* Fortran
* Matillion
* Databricks(?)

## Requirements
* Ease of use:
* Flexibility:
* High in-built feature support:
* Plugins/extensibility:
* High performance:
* Low/no cost:
* In-built metadata/modelling:
* (Nominal) DQ checks:

# DLT
[Delta Load Tool](https://dlthub.com/) (DLT)

DLT has great potential beyond simple ingestion. Within a data platform's total cost of ownership, storage is often the most cost effective.

[How DLT uses arrow.](https://dlthub.com/blog/how-dlt-uses-apache-arrow)
