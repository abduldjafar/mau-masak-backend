# Add Item To Persona Bucket

{% swagger method="post" path="/v1/admin/persona/bucket" baseUrl="" summary="" %}
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
{% endswagger %}

### Json Body

{% tabs %}
{% tab title="example" %}
```
// Some code
{
    "persona_id":"61c14e8baf04528f132196fd",
    "items_id":"61baa6b5caa4d756be66c79d"
}
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
