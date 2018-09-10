# REQ-translator
partof: REQ-purpose
###
Translator has two main roles. These are first; process the raw data, order book and trade history, into the ticker and exchange trust score. Secondly; storage of both the raw and processed data. The reason for this dual role (or triple role) is to prevent the translator from being divided into several 'services' thus increasing complexity. Therefore this portion of the project will be somewhat monolithic, or I prefer, menhirritic.

Therefore, the translator will be the `core` of Sammy with the data fetchers and website the periphery.

## Design
The `translator` **WILL** be a RESTful service written in [rust](https://www.rust-lang.org/) using the [actix](https://actix.rs/) framework. The RESTful API is [here](../api.raml). Actors will be used to construct the operation of the system. Here we have a replication of a networked system with nodes communicating with one another, only in this case it'll be actors messaging within the process space.

Each actor within the system will be specialized to handle a specific task. Therefore the system can be built in stages with new features added as new actors.

`translate` will receive push updates for new data on its RESTful API. This will be its main source of new market information. It will have an internal timer which will process the aggregated data each term cycle and store this in a database.

## Inputs
New market data posts on the RESTful API. Sorted by the URI path.

## Outputs
Data written to various DB's.
1. Term data **WILL** be written to Postgresql.
2. Raw trade history data **WILL** be written to Postgresql.
3. Raw order book data may be written to Mongo DB. This will have to be decided later.

# SPC-RESTful
partof: REQ-translator
###
Implementation of the [RESTful API](../api.raml). The primary input.

# SPC-collect
partof: REQ-translator
###
Internal component that collates that data received from [RESTful](#spc-restful) and stores it in an internal cache. This cache is then used to remove any duplicates and to determine which data (if any) of the newly added data set is truly new and worthy for being transmitted elsewhere within.

# SPC-term-calculate
partof: REQ-translator
###
Maintains calculation periods. Any data received is added to the currently active calculation period. When the period is due, the results are passed on and a new period commences.

# SPC-store
partof: REQ-translator
###
The output component of this project. Data, calculated and raw, is stored to backing services.
