# login

{% swagger method="post" path="/v1/admin/login" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="body" name="body" type="json" required="true" %}

{% endswagger-parameter %}
{% endswagger %}

{% tabs %}
{% tab title="Json Body" %}
```javascript
{
    "email":"admin@mico.earth",
    "password":""
}
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}

{% tabs %}
{% tab title="Example" %}
```
curl --location --request POST 'http://localhost:8080/v1/admin/login' \
--header 'Content-Type: application/json' \
--data-raw '{
    "email":"admin@mico.earth",
    "password":""
}'
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
