# Get All Item For Persona

{% swagger method="get" path="/v1/admin/persona/bucket" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="header" name="Authorization" required="true" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="persona_id" required="true" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="limit" type="int" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="page" type="int" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": [
            {
                "_id": "61c15696cc00a3aa62b425ea",
                "items_id": "61baa6b5caa4d756be66c79d",
                "items": {
                    "_id": "61baa6b5caa4d756be66c79d",
                    "vendor_id": "61b1cc7bf051f7bd5d223ddc",
                    "vendor_description": "",
                    "brand_name": "Khifana",
                    "house_room": "61aeea50b6d7aaff9c0ce5a9",
                    "product_category": "61aeea5ab6d7aaff9c0ce5aa",
                    "product_sub_category": "61af0650162c2c0bee83dd19",
                    "product_name": "Products With 1 Variants",
                    "price": 0,
                    "stripe_price_id": "",
                    "product_stripe_id": "",
                    "units": 0,
                    "image": "",
                    "mico_score": {
                        "co2": 0.18,
                        "water": 7,
                        "non_recyle": false,
                        "earth_waste_in_years": 1000,
                        "decomposition": 0
                    },
                    "mico_values": {
                        "vegan": {
                            "active": true,
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
                            "active": true,
                            "value": ""
                        },
                        "recyclable": {
                            "active": false,
                            "value": ""
                        },
                        "recycled": {
                            "active": false,
                            "value": ""
                        },
                        "donation": {
                            "active": false,
                            "value": ""
                        },
                        "ethically_produced": {
                            "active": true,
                            "value": ""
                        }
                    },
                    "product_description": "{\"time\":1639651022147,\"blocks\":[{\"id\":\"q3oaO3pVnK\",\"type\":\"paragraph\",\"data\":{\"text\":\"test\"}}],\"version\":\"2.22.2\"}",
                    "ingredients_materials": "Hello there, ini adalah ingredients and materials!",
                    "howto_handle": "",
                    "manufacturing_address": "",
                    "shipping_from": "Batam",
                    "shipping_method": "Mail",
                    "shipping_courier": "JNE",
                    "shipping_cost": 20,
                    "shipping_cost_stripe_id": "price_1K79vxF8sq9RvyoVgtsvJnSQ",
                    "shipping_method_stripe_id": "prod_KmjOtRRU0T8Wyb",
                    "delivery_time": "3 Hours of work",
                    "items_sku": [
                        {
                            "_id": "61baa71fcaa4d756be66c79e",
                            "sku_id": "PW1V-1",
                            "items_id": "61baa6b5caa4d756be66c79d",
                            "items_name": "Products With 1 Variants",
                            "color": "",
                            "size": "",
                            "stock": -3,
                            "sold": 3,
                            "price": 200,
                            "stripe_price_id": "price_1K7BR8F8sq9RvyoVkr0As4Kz",
                            "product_stripe_id": "prod_KmjMbUvuCun912",
                            "active": false,
                            "vendor_product_sku": "",
                            "pre_discounted_price": 0,
                            "created_at": "2021-12-16T02:40:31.343Z"
                        }
                    ],
                    "star": 0,
                    "created_at": "2021-12-16T02:38:45.78Z",
                    "type_of_packaging": "Recycled Cardboard",
                    "kg_of_packaging": 0.05,
                    "minimum_order_quantity": 1,
                    "weight": 0,
                    "pictures": [
                        "https://storage.googleapis.com/mico_project/item/61baa6b5caa4d756be66c79d/the_faunt-wallpaper-3840x2160.jpg"
                    ],
                    "details_category": {
                        "house_room": "",
                        "category": "",
                        "sub_category": ""
                    },
                    "published": true,
                    "average_yearly_consumption": 5,
                    "decomposition": 0,
                    "made_in": "Indonesia"
                },
                "persona_id": "61c14e8baf04528f132196fd"
            }
        ],
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}
{% endswagger %}
