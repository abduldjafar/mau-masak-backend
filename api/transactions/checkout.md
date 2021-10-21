# Checkout

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
        "data": {
            "transaction_id": "",
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
                "payment_gateway_shipping_id": "prod_KPd1tUq6Yl8puF"
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
        },
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
{}
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
