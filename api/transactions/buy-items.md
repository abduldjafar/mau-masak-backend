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
// this part can get from Checkout request responses
{
    "cart_items": [
        {
            "_id": "61659449dfd6529f41c6fcbf",
            "user_id": "61640e9022958d4cdbd91ee1",
            "items_id": "616591c6dfd6529f41c6fcbc",
            "items_colour": "red",
            "items_size": "xl",
            "count": 10,
            "price_idin_stripe": "price_1JjlImF8sq9RvyoVx3tYEvug",
            "user_stripe_id": "cus_KO7lOem6Z4A04p",
            "product_name": "",
            "price": 0,
            "mico_score": {
                "co2": 0,
                "water": 0,
                "non_recyle": false
            }
        }
    ],
    "shipping_address": "asoi",
    "shipping_methods": {
        "kind": "J&T",
        "price": 235,
        "payment_gateway_shipping_id": "price_1JknlIF8sq9RvyoVsznd6vpJ"
    },
    "card": "card",
    "MicoScore": {
        "co2": 10,
        "water": 10,
        "non_recyle": false
    },
    "ShoppingSummary": {
        "product_price": 544.5,
        "shipping_price": 235,
        "total_price": 779.5
    }
}
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
--data-raw '{
    "cart_items": [
        {
            "_id": "61659449dfd6529f41c6fcbf",
            "user_id": "61640e9022958d4cdbd91ee1",
            "items_id": "616591c6dfd6529f41c6fcbc",
            "items_colour": "red",
            "items_size": "xl",
            "count": 10,
            "price_idin_stripe": "price_1JjlImF8sq9RvyoVx3tYEvug",
            "user_stripe_id": "cus_KO7lOem6Z4A04p",
            "product_name": "",
            "price": 0,
            "mico_score": {
                "co2": 0,
                "water": 0,
                "non_recyle": false
            }
        }
    ],
    "shipping_address": "asoi",
    "shipping_methods": {
        "kind": "J&T",
        "price": 235,
        "payment_gateway_shipping_id": "price_1JknlIF8sq9RvyoVsznd6vpJ"
    },
    "card": "card",
    "MicoScore": {
        "co2": 10,
        "water": 10,
        "non_recyle": false
    },
    "ShoppingSummary": {
        "product_price": 544.5,
        "shipping_price": 235,
        "total_price": 779.5
    }
}'
```
{% endtab %}
{% endtabs %}
