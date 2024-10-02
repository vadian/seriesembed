# SeriesEmbed
Forked from EmSeries:


Documentation: [seriesembed - Rust](https://docs.rs/seriesembed/0.5.0/seriesembed/)

SeriesEmbed is an Embedded Time Series database. It is designed for small-scale applications which need to track time series data, but on a scale that does not justify extra database services.

It is intended for eventual use in applications that send and receive data multiple times per second from embedded sensors via SignalK.


## Features

*   Open a time series file directly in your application
*   Add, update, read, and delete records with arbitrary json-friendly structure
*   Search for records by timestamp and optional tags

## Future Plans

*   Indexing based on time and tags
*   Support databases larger than memory
*   Multi-process safety

The actual extent of the features implemened will depend on how I and any others decide to use them.
