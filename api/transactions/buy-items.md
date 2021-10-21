# Buy Items

{% swagger method="post" path="/v1/users/transaction/buy" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="header" name="Authorization" required="true" type="" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": "https://checkout.stripe.com/pay/cs_test_b1DP5h1CR3wP9cNTbTdI2lSPACjesmZu0h6IGJXBAQyfCSChgS75YpltCD#fidkdWxOYHwnPyd1blpxYHZxWjA0T210bTNDPXZ0PFdzfGpTSG9JbTdTZExrajVLbzQ0Nn80RGZyV1BHcWdWcXdHTm5xMmNQUEBtNnQzSEFIc1RKPXVpVUtOS19DN2BfUnJzcW1dfUhfQmBpNTU1MzY0c29XYycpJ2N3amhWYHdzYHcnP3F3cGApJ2lkfGpwcVF8dWAnPydocGlxbFpscWBoJyknYGtkZ2lgVWlkZmBtamlhYHd2Jz9xd3BgeCUl",
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}

{% swagger-response status="201" description="" %}
```javascript
{
    // Response
}
```
{% endswagger-response %}
{% endswagger %}

### Body request

{% tabs %}
{% tab title="Json Body" %}
```
//
{}
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}

### Example

{% tabs %}
{% tab title="curl" %}
```
curl --location --request POST 'http://localhost:8080/v1/users/transaction/buy' \
--header 'Authorization: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNjQwZTkwMjI5NThkNGNkYmQ5MWVlMSIsImVtYWlsIjoicnV0cmV0dGFzb2ljcnUtNzEwM0B5b3BtYWlsLmNvbSIsImRhdGF0eXBlIjoiYyIsImV4cCI6MTYzNDYzODcwNH0.V6v0e7kMNoDlMeBmM2t2RkJ18vT2SB2Su4boviYYDfA' \
--header 'Content-Type: application/json' \
--data-raw '{}'
```
{% endtab %}
{% endtabs %}
