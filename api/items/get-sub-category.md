# Get Sub Category

{% api-method method="get" host="https://api.cakes.com" path="/v1/house\_part/sub\_category" %}
{% api-method-summary %}
Get Cakes
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
{% api-method-parameter name="category\_id" type="string" required=true %}
The API will do its best to find a cake matching the provided recipe.
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
                "_id": "615c3c9d8c0f215d68379f4e",
                "house_part_id": "615a5a8b6f16322e92208508",
                "category_id": "615c3b21809e49eabc865cd9",
                "name": "asoeok"
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



