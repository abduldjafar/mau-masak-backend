# Get Items In Cart

{% swagger baseUrl="https://api.cakes.com" path="/v1/items/cart" method="get" summary="Get Cakes" %}
{% swagger-description %}
This endpoint allows you to get free cakes.
{% endswagger-description %}

{% swagger-parameter in="header" name="Authentication" type="string" %}
Authentication token to track down who is emptying our stocks.
{% endswagger-parameter %}

{% swagger-parameter in="query" name="limit" type="integer" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="page" type="integer" %}
Whether the cake should be gluten-free or not.
{% endswagger-parameter %}

{% swagger-response status="200" description="Cake successfully retrieved." %}
```
{
    "responses": {
        "code": 200,
        "data": [
            {
                "_id": "615e990a9b9abe9f1a164c05",
                "user_id": "6154d3810fd0274fa41e993c",
                "items_id": "615d7aee24605684a912f493",
                "items_colour": "red",
                "items_size": "xl",
                "count": 1,
                "product_name": "shampo aduhai v3",
                "price": 544.5,
                "mico_score": {
                    "co2": 10,
                    "water": 10,
                    "non_recyle": true
                }
            }
        ],
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}

{% swagger-response status="400" description="Could not find a cake matching this query." %}
```
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
```javascript
curl --location --request GET 'http://localhost:8080/v1/items/cart?page=0&limit=5' \
--header 'Authorization: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNTRkMzgxMGZkMDI3NGZhNDFlOTkzYyIsImVtYWlsIjoiNXlvdXNlZnNhbG1hbmFAaXNlb3ZlbHMuY29tIiwiZGF0YXR5cGUiOiIiLCJleHAiOjE2MzM2Njk0MTd9.vpjZ6hS7hu_R_9SGmanDDmem2Y9VkodHsF6yJtDKPL4'
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
