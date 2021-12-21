# Get All Persona

{% swagger method="get" path="/v1/admin/persona" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="header" name="Authorization" required="true" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="limit" type="int" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="page" type="int" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": [
            {
                "_id": "61c14e89af04528f132196f9",
                "persona_name": "asoi",
                "persona_number": 15,
                "persona_bucket_id": ""
            },
            {
                "_id": "61c14e8aaf04528f132196fb",
                "persona_name": "",
                "persona_number": 0,
                "persona_bucket_id": ""
            },
            {
                "_id": "61c14e8baf04528f132196fd",
                "persona_name": "",
                "persona_number": 0,
                "persona_bucket_id": ""
            }
        ],
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}
{% endswagger %}
