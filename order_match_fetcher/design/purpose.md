# REQ-order-match-fetching
partof: REQ-exchange-data-fetching
###
One half of the raw data fetching. This module will focus on the fetching of order book history.

## Minimum viable product
Connect to three exchanges and fetching trade history in the specified crypto currencies. Fetching will be done in a regular manner with the intention of not losing any data if possible. Therefore there **SHOULD** not be any gaps in the record being fetched. But since there is no SLA with the exchanges it's impossible to make it a hard requirement.

## Operation
The program will connect to the exchanges public web API's and send data fetch requests. In response, the exchange will emit the trade history data. Upon receipt, the program will convert the data into a standard format and [post](https://en.wikipedia.org/wiki/POST_(HTTP)) it to the translator.

# SPC-exchange-handlers
partof: REQ-exchange-data-fetching
###
Modules for handling the nitty gritty of exchange API's and the fetching logic.

# SPC-kraken
partof: REQ-exchange-handlers
###
Implement trade history fetching.

- [[.fetch]]: Carryout the actual fetch.
- [[.polling]]: Polling loop using the returned ID.
- [[.throttling]]: Handle exchange throttling gracefully. We don't want to hammer them and get banned.
- [[.conversion]]: Convert the returned data into a standardized internal format for transmission to the rest of the Sammy system.


# SPC-OKEx
partof: REQ-exchange-handlers
###
Implement trade history fetching.

# SPC-bitbank
partof: REQ-exchange-handlers
###
Implement trade history fetching.

# SPC-transmission
partof: REQ-order-match-fetching
###
Transmit the converted data to the rest of the system.
