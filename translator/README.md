Translator
===

Core part of the `sammy` project.


## RESTful API
The RESTful API is decribed in detail in the [api.raml](api.raml) file.

The API endpoints are:

## Testing the API
Assuming `translator` has been bound to localhost;

For `/trade_history`
```console
curl http://127.0.0.1:8080/trade_history
```
Output should be a list of asset pairs and exchanges.
