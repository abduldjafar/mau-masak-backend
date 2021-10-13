# Search Items

{% swagger method="get" path="/v1/items/search" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="query" name="query" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="vendor_id" %}

{% endswagger-parameter %}

{% swagger-parameter in="header" name="Authorization" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": [
            {
                "_id": "616591c6dfd6529f41c6fcbc",
                "amount": [
                    {
                        "colour": "red",
                        "count": 12,
                        "size": "xl"
                    },
                    {
                        "colour": "black",
                        "count": 10,
                        "size": "xl"
                    }
                ],
                "buying_details": {
                    "country_origin": "xx",
                    "delivery_packaging": "xx",
                    "delivery_time": "xx"
                },
                "created_at": "2021-10-12T20:46:45.936+07:00",
                "house_room": "bathroom",
                "mico_score": {
                    "co2": 10,
                    "non_recyle": true,
                    "water": 10
                },
                "mico_values": {
                    "is_co2_exist": false,
                    "is_recycling": true,
                    "one_hundred_percent_natureal": true,
                    "palm_oil_free": false,
                    "plastic_free": true,
                    "vegan": true
                },
                "price": 544.5,
                "product_category": "hair",
                "product_name": "shampo aduhai v3",
                "product_overview": "",
                "product_stripe_id": "prod_KOYP0diJacDKKF",
                "product_sub_category": "shampoo",
                "star": 0,
                "stripe_price_id": "price_1JjlImF8sq9RvyoVx3tYEvug",
                "vendor_id": "61525154347c025c4cdc29c0"
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
        "code": 200,
        "data": null,
        "isError": true,
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
curl --location --request GET 'http://localhost:8080/v1/items/search?query=sh&vendor_id=xxxx' \
--header 'Authorization: xxxxxxxx'
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
