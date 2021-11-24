# Get Items

{% swagger baseUrl="https://api.cakes.com" path="/v1/items" method="get" summary="Get Items" %}
{% swagger-description %}
This endpoint allows you to get free cakes.
{% endswagger-description %}

{% swagger-parameter in="header" name="Authorization" type="string" %}
Authentication token to track down who is emptying our stocks.
{% endswagger-parameter %}

{% swagger-parameter in="query" name="product_sub_category" type="string" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="house_room" type="string" %}
The API will do its best to find a cake matching the provided recipe.
{% endswagger-parameter %}

{% swagger-parameter in="query" name="product_category" type="string" %}
Whether the cake should be gluten-free or not.
{% endswagger-parameter %}

{% swagger-parameter in="query" name="limit" type="int" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="page" type="int" %}

{% endswagger-parameter %}

{% swagger-response status="200" description="Cake successfully retrieved." %}
```
{
    "responses": {
        "code": 200,
        "data": {
            "detail_category": {
                "category": {
                    "id": "617bc6e7b6028c6baeb8bace",
                    "value": "Hair"
                },
                "house_part": {
                    "id": "617bc57c6039d894317b2b71",
                    "value": "Bathroom"
                },
                "sub_category": {
                    "id": "617bc8bdb6028c6baeb8bad3",
                    "value": "Shampoo"
                }
            },
            "items": [
                {
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
                }
            ]
        },
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}

{% swagger-response status="404" description="Could not find a cake matching this query." %}
```
{    "message": "Ain't no cake like that."}
```
{% endswagger-response %}
{% endswagger %}

### Example

{% tabs %}
{% tab title="curl" %}
```javascript
curl --location --request GET 'http://localhost:8080/v1/items?house_room=bathroom&product_category=hair&product_sub_category=shampoo' \
--header 'Authorization: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNTRkMzgxMGZkMDI3NGZhNDFlOTkzYyIsImVtYWlsIjoiNXlvdXNlZnNhbG1hbmFAaXNlb3ZlbHMuY29tIiwiZGF0YXR5cGUiOiIiLCJleHAiOjE2MzM2MDA2NzN9.YWNKD6m-n0YbYfomGIChSeHr-HO-kio7QOHQrhCMIGE'
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
