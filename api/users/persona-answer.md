# persona answer

{% swagger method="post" path="/v1/users/persona/answer" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="header" name="Authorization" required="true" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": [
            {
                "_id": "61c179ef8a8cb50b9856d8eb",
                "items_id": "",
                "items": {
                    "_id": "000000000000000000000000",
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
                    "image": "",
                    "mico_score": {
                        "co2": 0,
                        "water": 0,
                        "non_recyle": false,
                        "earth_waste_in_years": 0,
                        "decomposition": 0
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
                        "recycled": {
                            "active": false,
                            "value": ""
                        },
                        "donation": {
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
                    "created_at": "0001-01-01T00:00:00Z",
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
                    "average_yearly_consumption": 0,
                    "decomposition": 0,
                    "made_in": ""
                },
                "persona_id": "61c179ee8a8cb50b9856d8ea"
            },
            {
                "_id": "61c180e8b5b5d5eb8a28fbee",
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
                "persona_id": "61c179ee8a8cb50b9856d8ea"
            }
        ],
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}
{% endswagger %}

### Json Body

{% tabs %}
{% tab title="example" %}
```
{
    "answer":[
        {
            "question_id":"5",
            "question_answer":"a",
            "question_sentences":"",
            "answer_sentences":""
        }
    ]
}
// because currently , we just get answer from questions numnber 5. so the question_id should be "5"
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
