# create tags

{% swagger method="post" path="" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-response status="200: OK" description="" %}
```javascript
{
    "responses": {
        "code": 200,
        "data": "6197724102304889d352210d",
        "isError": false,
        "message": "success"
    }
}
```
{% endswagger-response %}
{% endswagger %}

{% tabs %}
{% tab title="Json Body" %}
```
{
    "name":"asoi"
}
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}

### Example

{% tabs %}
{% tab title="curl" %}
```
curl --location --request POST 'http://localhost:8080/v1/users/content/category/tags' \
--header 'Authorization: xxxxx' \
--header 'Content-Type: application/json' \
--data-raw '{
    "name":"asoi"
}'
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
