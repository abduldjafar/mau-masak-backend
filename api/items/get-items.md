# Get Items

{% api-method method="get" host="https://api.cakes.com" path="/v1/items" %}
{% api-method-summary %}
Get Items
{% endapi-method-summary %}

{% api-method-description %}
This endpoint allows you to get free cakes.
{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-headers %}
{% api-method-parameter name="Authorization" type="string" required=true %}
Authentication token to track down who is emptying our stocks.
{% endapi-method-parameter %}
{% endapi-method-headers %}

{% api-method-query-parameters %}
{% api-method-parameter name="product\_sub\_category" type="string" required=false %}

{% endapi-method-parameter %}

{% api-method-parameter name="house\_room" type="string" %}
The API will do its best to find a cake matching the provided recipe.
{% endapi-method-parameter %}

{% api-method-parameter name="product\_category" type="boolean" %}
Whether the cake should be gluten-free or not.
{% endapi-method-parameter %}
{% endapi-method-query-parameters %}
{% endapi-method-request %}

{% api-method-response %}
{% api-method-response-example httpCode=200 %}
{% api-method-response-example-description %}
Cake successfully retrieved.
{% endapi-method-response-example-description %}

```
{
    "responses": {
        "code": 200,
        "data": [
            {
                "_id": "615d7361a7404a8a4a7631e5",
                "house_room": "bathroom",
                "product_category": "hair",
                "product_sub_category": "shampoo",
                "product_name": "shampo aduhai",
                "price": 300.23,
                "mico_score": {
                    "co2": 90,
                    "water": 80,
                    "non_recyle": true
                },
                "mico_values": {
                    "one_hundred_percent_natureal": true,
                    "plastic_free": true,
                    "is_recycling": true,
                    "palm_oil_free": false,
                    "vegan": true,
                    "is_co2_exist": false
                },
                "product_overview": "",
                "buying_details": {
                    "country_origin": "xx",
                    "delivery_time": "xx",
                    "delivery_packaging": "xx"
                },
                "amount": [
                    {
                        "colour": "red",
                        "size": "xl",
                        "count": 12
                    },
                    {
                        "colour": "black",
                        "size": "xl",
                        "count": 10
                    }
                ],
                "star": 0,
                "created_at": "2021-10-06T09:58:57.452Z"
            }
        ],
        "isError": false,
        "message": "success"
    }
}
```
{% endapi-method-response-example %}

{% api-method-response-example httpCode=404 %}
{% api-method-response-example-description %}
Could not find a cake matching this query.
{% endapi-method-response-example-description %}

```
{    "message": "Ain't no cake like that."}
```
{% endapi-method-response-example %}
{% endapi-method-response %}
{% endapi-method-spec %}
{% endapi-method %}



