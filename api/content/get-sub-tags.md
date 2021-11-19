# get sub tags

{% swagger method="get" path="/v1/users/content/category/sub_tags" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="query" name="limit" type="int" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="page" type="int" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="tags" type="string" required="true" %}
id tags
{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": [
            {
                "_id": "6195058251b0b2f1618b6a7f",
                "conten_tags_id": "6195058251b0b2f1618b6a7e",
                "name": "sub-tags-1-1",
                "picture": ""
            }
        ],
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}
{% endswagger %}
