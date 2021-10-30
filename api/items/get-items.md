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
