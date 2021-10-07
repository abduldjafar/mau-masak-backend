# Get Items In Cart

{% api-method method="get" host="https://api.cakes.com" path="/v1/items/cart" %}
{% api-method-summary %}
Get Cakes
{% endapi-method-summary %}

{% api-method-description %}
This endpoint allows you to get free cakes.
{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-headers %}
{% api-method-parameter name="Authentication" type="string" required=true %}
Authentication token to track down who is emptying our stocks.
{% endapi-method-parameter %}
{% endapi-method-headers %}

{% api-method-query-parameters %}
{% api-method-parameter name="limit" type="integer" required=false %}

{% endapi-method-parameter %}

{% api-method-parameter name="page" type="integer" %}
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
                "_id": "615e990a9b9abe9f1a164c05",
                "user_id": "6154d3810fd0274fa41e993c",
                "items_id": "615d7aee24605684a912f493",
                "items_colour": "red",
                "items_size": "xl",
                "count": 1
            }
        ],
        "isError": false,
        "message": "success"
    }
}
```
{% endapi-method-response-example %}

{% api-method-response-example httpCode=400 %}
{% api-method-response-example-description %}
Could not find a cake matching this query.
{% endapi-method-response-example-description %}

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
{% endapi-method-response-example %}
{% endapi-method-response %}
{% endapi-method-spec %}
{% endapi-method %}



