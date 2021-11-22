# get transaction by customer\_id

{% swagger method="get" path="/admin/transaction" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="query" name="limit" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="page" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="customer_id" required="true" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": [
            {
                "_id": "616d4b67057bbdfe7af677b9",
                "user_id": "61640e9022958d4cdbd91ee1",
                "items_in_cart": [
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
                "product_count": 1,
                "paid": false,
                "total_paid": 779.5,
                "product_paid": 0,
                "shipping_paid": 0,
                "url_checkout": "https://checkout.stripe.com/pay/cs_test_b1DP5h1CR3wP9cNTbTdI2lSPACjesmZu0h6IGJXBAQyfCSChgS75YpltCD#fidkdWxOYHwnPyd1blpxYHZxWjA0T210bTNDPXZ0PFdzfGpTSG9JbTdTZExrajVLbzQ0Nn80RGZyV1BHcWdWcXdHTm5xMmNQUEBtNnQzSEFIc1RKPXVpVUtOS19DN2BfUnJzcW1dfUhfQmBpNTU1MzY0c29XYycpJ2N3amhWYHdzYHcnP3F3cGApJ2lkfGpwcVF8dWAnPydocGlxbFpscWBoJyknYGtkZ2lgVWlkZmBtamlhYHd2Jz9xd3BgeCUl",
                "created_at": "2021-10-18T10:24:39.188Z",
                "paid_date": "0001-01-01T00:00:00Z",
                "shipping_methods": "",
                "shipping_address": "asoi",
                "payment_gateway_id": "pi_3Jlt0NF8sq9RvyoV1pZWbtXR"
            }
        ],
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
        "isError": true,
        "message": "from server"
    }
}
```
{% endswagger-response %}
{% endswagger %}
