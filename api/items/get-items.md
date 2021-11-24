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
                    "items_sku": null,
                    "star": 0,
                    "created_at": "2021-10-29T10:38:55.771Z",
                    "type_of_packaging": "HDPE",
                    "weight": 9,
                    "pictures": ["https://storage.googleapis.com/mico_project/item/617bcf408527296121bdc0b8/ayurvedic.webp"],
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
