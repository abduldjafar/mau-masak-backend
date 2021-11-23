# get persona answer

{% swagger method="get" path="/v1/user/persona_answer" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="query" required="true" name="question_number_one" type="string" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="question_number_two" required="true" type="string" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="question_number_three" required="true" type="string" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="question_number_four" required="true" type="string" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="question_number_five" required="true" type="string" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": {
            "question_number_one": "a",
            "question_number_two": "b",
            "question_number_three": "c",
            "question_number_four": "d",
            "question_number_five": "e",
            "items": [
                {
                    "_id": "617e4c43117bd704d9e6051b",
                    "vendor_id": "",
                    "vendor_description": "",
                    "brand_name": "Minimal organics",
                    "house_room": "617bc57c6039d894317b2b71",
                    "product_category": "617bc6e7b6028c6baeb8bace",
                    "product_sub_category": "617bc8bdb6028c6baeb8bad3",
                    "product_name": "Hair Soap Ayurvedic 80g",
                    "price": 14,
                    "stripe_price_id": "price_1JpsT6F8sq9RvyoVynrMgfui",
                    "product_stripe_id": "prod_KUsDXx8sxd8FuZ",
                    "mico_score": {
                        "co2": 0.06,
                        "water": 2.81,
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
                            "active": true,
                            "value": "90%"
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
                    "product_overview": "Rich in saponins, vitamins and antioxidants that make your hair shiny improving hair’s texture. Nourishes follicles. How to use: Make sure to keep your shampoo dry before and after every use and place it on soap dish with drainage to remove any excess water. \n If you’re using your shampoo on the go, make sure to let it dry before packing. To prevent contamination and breakage, keep your shampoo in a small box or hard container for travelling. For all hair types.",
                    "buying_details": {
                        "country_origin": "Switzerland",
                        "delivery_time": "3-5 business days",
                        "delivery_packaging": "Sugarcane Bagasse"
                    },
                    "howto_handle": "",
                    "manufacturing_address": "",
                    "shipment_from_address": "",
                    "amount": null,
                    "star": 0,
                    "created_at": "2021-10-29T10:38:55.771Z",
                    "type_of_packaging": "HDPE",
                    "weight": 9,
                    "pictures": [
                        "https://storage.googleapis.com/mico_project/item/617bcf408527296121bdc0b8/ayurvedic.webp"
                    ],
                    "details_category": {
                        "house_room": "",
                        "category": "",
                        "sub_category": ""
                    }
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
