# EmSeries

[![CircleCI](https://circleci.com/gh/luminescent-dreams/emseries.svg?style=svg)](https://circleci.com/gh/luminescent-dreams/emseries)

Documentation: [emseries - Rust](https://docs.rs/emseries/0.3.0/emseries/)

EmSeries is an Embedded Time Series database. It is designed for small-scale applications which need to track time series data, but on a scale that does not justify extra database services. I use it for [Fitnesstrax](https://github.com/luminescent-dreams/fitnesstrax), which keeps track of information recorded only a few times a day.

I intend to use it eventually for an application that receives data every few seconds from sensors scattered around my house.

## Features

*   Open a time series file directly in your application
*   Add, update, read, and delete records with arbitrary json-friendly structure
*   Search for records by timestamp and optional tags

## Future Plans

*   Indexing based on time and tags
*   Support databases larger than memory
*   Multi-process safety

The actual extent of the features implemened will depend on how I and any others decide to use them.
