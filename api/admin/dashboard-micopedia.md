# dashboard micopedia

{% swagger method="get" path="/v1/admin/dasboard_micopedia" baseUrl="" summary="" %}
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
                    "total_micopedia_articles": 2,
                    "total_admin_articles": 0,
                    "total_user_articles": 2,
                    "total_article_reader": 0
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
        "code": 200,
        "data": null,
        "isError": true,
        "message": "from server"
    }
}
```
{% endswagger-response %}
{% endswagger %}
