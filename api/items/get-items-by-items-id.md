# get items by items id

{% swagger method="get" path="/v1/items/:id" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="path" name="id" required="true" type="string" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": {
            "_id": "619e3a8d4f6e52d8ad933be4",
            "vendor_id": "",
            "vendor_description": "",
            "brand_name": "",
            "house_room": "",
            "product_category": "",
            "product_sub_category": "",
            "product_name": "",
            "price": 0,
            "stripe_price_id": "",
            "product_stripe_id": "",
            "units": 0,
            "mico_score": {
                "co2": 0,
                "water": 0,
                "non_recyle": false
            },
            "mico_values": {
                "vegan": {
                    "active": false,
                    "value": ""
                },
                "plastic_free": {
                    "active": false,
                    "value": ""
                },
                "organic": {
                    "active": false,
                    "value": ""
                },
                "glycerin_free": {
                    "active": false,
                    "value": ""
                },
                "palm_oil_free": {
                    "active": false,
                    "value": ""
                },
                "local": {
                    "active": false,
                    "value": ""
                },
                "compostable": {
                    "active": false,
                    "value": ""
                },
                "biodegradable": {
                    "active": false,
                    "value": ""
                },
                "recyclable": {
                    "active": false,
                    "value": ""
                },
                "ethically_produced": {
                    "active": false,
                    "value": ""
                }
            },
            "product_description": "",
            "ingredients_materials": "",
            "howto_handle": "",
            "manufacturing_address": "",
            "shipping_from": "",
            "shipping_method": "",
            "shipping_courier": "",
            "shipping_cost": 0,
            "delivery_time": "",
            "items_sku": [
                {
                    "_id": "6198535862fd76405447697c",
                    "items_id": "619e3a8d4f6e52d8ad933be4",
                    "colour": "Black",
                    "size": "L",
                    "stock": 10,
                    "sold": 0,
                    "price": 200,
                    "stripe_price_id": "price_1Jxid6F8sq9RvyoV2nX8WSZK",
                    "product_stripe_id": "prod_KcyaOwUvLPENZL",
                    "active": false,
                    "created_at": "2021-11-20T01:46:00.737Z"
                }
            ],
            "star": 0,
            "created_at": "2021-11-24T13:13:49.32Z",
            "type_of_packaging": "",
            "kg_of_packaging": 0,
            "minimum_order_quantity": 0,
            "weight": 0,
            "pictures": null,
            "details_category": {
                "house_room": "",
                "category": "",
                "sub_category": ""
            },
            "published": false,
            "average_yearly_consumption": 0
        },
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}
{% endswagger %}
