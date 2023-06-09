# stx_test


**Business requirements**

Standalone module that makes automated withdrawals of STX token on the balance from an account on okcoin exchange. 
Withdrawals are mode to a pre-saved STX address. 
There should be configurable options:
- if greater certain amount of STX on the balance then total available amount can be sent
- there will be 2 addresses to send to, module should use them in turn

[okcoin API](https://www.okcoin.com/docs-v5/)

Withdrawal issue

POST: {"msg":"Invalid Sign","code":"50113"}

The error occurs when making a POST request. The request was not properly authenticated. Possible reasons:

- Incorrect API key. API key has the necessary permissions to perform withdrawal action and works with GET.
- Incorrect timestamp. (Works with GET)
- Incorrect signature. (Works with GET) The signature is created by hashing the pre-signed string with the API secret key using the HMAC-SHA256 algorithm. Make sure that the pre-signed string contains all the required parameters and that they are in the correct order.

Possible solutions:

- Double-check the API key, secret key, and passphrase.
- Verify that the timestamp in the request header is correct.
- Verify that the signature is correctly generated and matches the one expected by the server.
- Check that the request parameters are correctly formatted.
- Check API documentation for any specific requirements or restrictions for the requested action.

It is also possible that the error is caused by a temporary issue with the server.