# get bad items by sub\_category

{% swagger method="get" path="/v1/items/bad" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="query" name="product_sub_category" required="true" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": {
            "detail_category": null,
            "items": [
                {
                    "_id": "6181d16d9d5b8cedd1d5e157",
                    "vendor_id": "",
                    "vendor_description": "",
                    "brand_name": "",
                    "house_room": "617bc5d76039d894317b2b73",
                    "product_category": "617bc79ab6028c6baeb8bad2",
                    "product_sub_category": "617d49d05fd4974ba1e74e3f",
                    "product_name": "one-time lunch boxes",
                    "price": 0,
                    "stripe_price_id": "",
                    "product_stripe_id": "",
                    "units": 0,
                    "image": "",
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
                    "shipping_cost_stripe_id": "",
                    "shipping_method_stripe_id": "",
                    "delivery_time": "",
                    "items_sku": null,
                    "star": 0,
                    "created_at": "2021-11-03T00:01:49.736Z",
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
                }
            ]
        },
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}
{% endswagger %}
