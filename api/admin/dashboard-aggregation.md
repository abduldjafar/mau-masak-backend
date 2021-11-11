# dashboard aggregation

{% swagger method="get" path="/v1/admin/dasboard_aggregation" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="header" name="Authorization" required="true" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": {
            "dashboard_overview": {
                "aggregation_summary": {
                    "new_order": 0,
                    "total_product_sold": 0,
                    "total_product": 0,
                    "total_total_category": 0,
                    "total_customer": 0,
                    "total_vendors": 0
                }
            }
        },
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
