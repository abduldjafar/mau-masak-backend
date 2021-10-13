# Checkout Selected Items

{% swagger method="post" path="/v1/users/transaction/select" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="header" name="Authorization" required="true" %}

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

### Body Requests

{% tabs %}
{% tab title="Json Body" %}
```javascript
{
    "charts_id":["xxxxx","yyyy"]
}
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
