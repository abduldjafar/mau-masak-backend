# Forgot Password

{% swagger method="post" path="/v1/users/forgot_password" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

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

### Body Json

{% tabs %}
{% tab title="example" %}
```
{
    "email":"abdul.haris.djafar@gmail.com"
}
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
