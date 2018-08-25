# REQ-purpose
Sammy will emit a ticker for various crypto/asset pairs on various exchanges. In addition to this Sammy will also analyze it's exchange feeds for manipulation thus producing an exchange trust score. These two pices of information will help investors make informed trading choices.

## Minimum Viable Product
The MVP **SHALL** connect to three exchanges and fetch trade history and order book data in a [regular manner](#exchange-data-fetching). The asset pairs being fetched **SHALL** be BTC/USD, ETH/USD, BTC/ETH and BNB/USD. Sammy **SHALL** store this data in a lossless form for possible future uses that are beyond the scope of this design document. Sammy **SHOULD** fetch this data consistently but allowances must be made due to the fact that there is no SLA with the exchanges in question; therefore there may be gaps in the stored record. Sammy **SHALL** transform the data feed into two outputs (broken down by asset pair and exchange), a [ticker](#ticker) and an [exchange trust score](#exchange-trust-score). Sammy **WILL** display this information via a web interface.

### Inputs
The inputs will be from exchange public API's and user actions on the website interface. 

The exchange inputs will be the raw data inputs;
- Trade history
- Order book
These inputs will be fetched for the four specified asset pairs, BTC/USD, ETH/USD, BTC/ETH and BNB/USD. The three [exchanges](#exchanges) have not yet been chosen.

The user inputs will be;
- Commands received on the web interface.
  * Clicking buttons.
  * Viewing web-pages.
  
### Outputs
Outputs will be in two forms, the first is the data being persisted and second the data being displayed to the user via the website.

Persisted data **SHALL** be of two types, raw and processed.

The persisted raw data **SHALL** be;
- Trade history
- Order book
The trade history and order book data **SHALL** be ordered by time and stored in a lossless format from receipt by the exchange. Therefore if the exchange, in the case of order books for example, does some form of data merging and presents the merged data; then this information is lost to Sammy and in no way shall it be reproduced. The act of output **SHALL** be the database write command. To view this data is beyond the scope of this design document. 

The persisted processed data **SHALL** be;
- Ticker
- Exchange trust score (over time)

Displayed data **SHALL** be;
- The ticker, divided by asset pair and exchange.
- Exchange trust score.
This data will be displayed by the [website](#website) portion of Sammy.

# REQ-exchange-data-fetching
partof: REQ-purpose
###
The component of Sammy which gets the raw data, order book and trade history, from the specified exchanges.

# REQ-translator
partof: REQ-purpose
###
Translator has two main roles. These are first; process the raw data, order book and trade history, into the ticker and exchange trust score. Secondly; storage of both the raw and processed data. The reason for this dual role (or triple role) is to prevent the translator from being divided into several 'services' thus increasing complexity. Therefore this portion of the project will be somewhat monolithic, or I prefer, menhirritic.

# REQ-website
partof: REQ-purpose
###
Display of the processed data to the user. Receive commands from the user on what of the processed data to display. This component **WILL** have read only access to the processed data store written to by the translator.
