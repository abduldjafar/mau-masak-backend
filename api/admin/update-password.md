# update password

{% swagger method="put" path="/v1/admin/password_profile" baseUrl="" summary="" %}
{% swagger-description %}

{% endswagger-description %}

{% swagger-parameter in="header" name="Authorization" required="true" %}

{% endswagger-parameter %}
{% endswagger %}

### Json Body

{% tabs %}
{% tab title="body" %}
```javascript
{
    "new_password":"67890",
    "old_password":"12345"
}
```
{% endtab %}

{% tab title="Second Tab" %}

{% endtab %}
{% endtabs %}
