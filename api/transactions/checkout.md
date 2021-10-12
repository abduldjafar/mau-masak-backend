# Checkout

{% swagger method="get" path="/v1/users/transaction" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="header" name="Authorization" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": "https://checkout.stripe.com/pay/cs_test_b1bJkS2pvWHqdbMFH2XTxfNnY0ll2piF9zh76dOeiaXYJWlaaFOSRtP2w3#fidkdWxOYHwnPyd1blpxYHZxWjA0T210bTNDPXZ0PFdzfGpTSG9JbTdTZExrajVLbzQ0Nn80RGZyV1BHcWdWcXdHTm5xMmNQUEBtNnQzSEFIc1RKPXVpVUtOS19DN2BfUnJzcW1dfUhfQmBpNTU1MzY0c29XYycpJ2N3amhWYHdzYHcnP3F3cGApJ2lkfGpwcVF8dWAnPydocGlxbFpscWBoJyknYGtkZ2lgVWlkZmBtamlhYHd2Jz9xd3BgeCUl",
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}

{% swagger-response status="400: Bad Request" description="" %}
```javascript
{
    "responses": {
        "code": 400,
        "data": null,
        "isError":true,
        "message": "from server"
    }
}
```
{% endswagger-response %}
{% endswagger %}

### Example

{% tabs %}
{% tab title="curl" %}
```
curl --location --request GET 'https://mico-backend-services-i5jta7dz4q-uc.a.run.app/v1/users/transaction' \
--header 'Authorization: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNjQwZTkwMjI5NThkNGNkYmQ5MWVlMSIsImVtYWlsIjoicnV0cmV0dGFzb2ljcnUtNzEwM0B5b3BtYWlsLmNvbSIsImRhdGF0eXBlIjoiYyIsImV4cCI6MTYzNDAzMzczM30.AUsU9zLS84D16M0aTyx6WbeJjiQH0i7SEviH5WHC2uM'
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
