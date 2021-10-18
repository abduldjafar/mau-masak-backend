# Delete Item In Cart

{% swagger baseUrl="https://api.cakes.com" path="/v1/items/cart" method="delete" summary="Get Cakes" %}
{% swagger-description %}
This endpoint allows you to get free cakes.
{% endswagger-description %}

{% swagger-parameter in="path" name="cart_id" type="string" %}
ID of the cake to get, for free of course.
{% endswagger-parameter %}

{% swagger-parameter in="header" name="Authorization" type="string" %}
Authentication token to track down who is emptying our stocks.
{% endswagger-parameter %}

{% swagger-response status="200" description="Cake successfully retrieved." %}
```
{
    "responses": {
        "code": 200,
        "data": null,
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
        "message": "message from server"
    }
}
```
{% endswagger-response %}
{% endswagger %}

### Example

{% tabs %}
{% tab title="curl" %}
```javascript
curl --location --request DELETE 'http://localhost:8080/v1/items/cart/615e88bcdccee8fb2da81441' \
--header 'Authorization: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjYxNTRkMzgxMGZkMDI3NGZhNDFlOTkzYyIsImVtYWlsIjoiNXlvdXNlZnNhbG1hbmFAaXNlb3ZlbHMuY29tIiwiZGF0YXR5cGUiOiIiLCJleHAiOjE2MzM2Njk0MTd9.vpjZ6hS7hu_R_9SGmanDDmem2Y9VkodHsF6yJtDKPL4'
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
