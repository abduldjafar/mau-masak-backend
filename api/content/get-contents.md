# Get Contents

{% swagger method="get" path="/v1/users/content" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="header" name="Authorization" required="true" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": [
            {
                "_id": "617270bde3e8216a121c99fa",
                "user_id": "61640e9022958d4cdbd91ee1",
                "user_type": "",
                "created_at": "0001-01-01T00:00:00Z",
                "body_content": "haloha"
            },
            {
                "_id": "617271d25867de1f57135fea",
                "user_id": "61640e9022958d4cdbd91ee1",
                "user_type": "",
                "created_at": "2021-10-22T08:09:54.97Z",
                "body_content": "haloha"
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
        "message": "success"
    }
}
```
{% endswagger-response %}
{% endswagger %}
