# dashboard vendor

{% swagger method="get" path="/v1/admin/dasboard_vendor" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="query" name="page" type="int" %}

{% endswagger-parameter %}

{% swagger-parameter in="query" name="limit" type="int" %}

{% endswagger-parameter %}

{% swagger-parameter in="header" name="Authorization" required="true" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": [
            {
                "location": "",
                "name": "",
                "total_product": 0,
                "total_product_sold": 0,
                "total_revenue": 0
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
