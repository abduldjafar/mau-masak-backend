# get tags

{% swagger method="get" path="/v1/users/content/category/tags" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="query" name="page" type="int" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="limit" type="int" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": [
            {
                "_id": "6195058251b0b2f1618b6a7e",
                "name": "tags-1",
                "picture": ""
            },
            {
                "_id": "61976f69ce8e7b667b058468",
                "name": "asoi",
                "picture": ""
            },
            {
                "_id": "6197724102304889d352210d",
                "name": "asoi",
                "picture": ""
            }
        ],
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}

{% swagger-response status="400: Bad Request" description="" %}
```javascript
{
    "responses": {
        "code": 400,
        "data": null,
        "isError": true,
        "message": "from server"
    }
}
```
{% endswagger-response %}
{% endswagger %}
