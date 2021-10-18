# Get Sub Category

{% swagger baseUrl="https://api.cakes.com" path="/v1/house_part/sub_category" method="get" summary="Get Cakes" %}
{% swagger-description %}
This endpoint allows you to get free cakes.
{% endswagger-description %}

{% swagger-parameter in="header" name="Authorization" type="string" %}
Authentication token to track down who is emptying our stocks.
{% endswagger-parameter %}

{% swagger-parameter in="query" name="category_id" type="string" %}
The API will do its best to find a cake matching the provided recipe.
{% endswagger-parameter %}

{% swagger-response status="200" description="Cake successfully retrieved." %}
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
{% endswagger-response %}

{% swagger-response status="404" description="Could not find a cake matching this query." %}
```
{    "message": "Ain't no cake like that."}
```
{% endswagger-response %}
{% endswagger %}

