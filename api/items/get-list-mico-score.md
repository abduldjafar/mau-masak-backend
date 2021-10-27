# Get list Mico Score

{% swagger method="get" path="/v1/items/mico_scores" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": [
            {
                "name": "HDPE",
                "per_amount_kg": 1,
                "co2_kg": 6.88,
                "water_liters": 312.73
            },
            {
                "name": "PET",
                "per_amount_kg": 1,
                "co2_kg": 3.44,
                "water_liters": 235
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
        "data":null,
        "isError": true,
        "message": "from server"
    }
}
```
{% endswagger-response %}
{% endswagger %}
