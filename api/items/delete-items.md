# Delete Items

{% swagger method="delete" path="/v1/items/:items_id" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="path" name="items_id" required="true" %}

{% endswagger-parameter %}

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

### Example

{% tabs %}
{% tab title="curl" %}
```
curl --location --request DELETE 'http://localhost:8080/v1/items/616507df8a2c8a08ea7dce4b' \
--header 'Authorization: xxxxxxxxx'
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
