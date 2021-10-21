# Select Cart

{% swagger method="post" path="/v1/items/cart/select" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="header" name="Authorization" required="true" %}

{% endswagger-parameter %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": null,
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

### Body Request

{% tabs %}
{% tab title="Json Body" %}
```
{
    "charts": [
        {
            "charts_id": "617170e90e25e988f41feca4",
            "selected": true
        }
    ]
}

// if before the chart was selected and want to unselected please put charts
id with false selected value
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
