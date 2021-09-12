# Save Users

{% api-method method="post" host="https://api-dev.mico.sg/v1" path="/users/save" %}
{% api-method-summary %}
save users
{% endapi-method-summary %}

{% api-method-description %}

{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-body-parameters %}
{% api-method-parameter name="" type="object" required=true %}
json body for create users
{% endapi-method-parameter %}
{% endapi-method-body-parameters %}
{% endapi-method-request %}

{% api-method-response %}
{% api-method-response-example httpCode=200 %}
{% api-method-response-example-description %}

{% endapi-method-response-example-description %}

```

```
{% endapi-method-response-example %}
{% endapi-method-response %}
{% endapi-method-spec %}
{% endapi-method %}

{% tabs %}
{% tab title="Json Body" %}
```text
{
    "email":"koteka@gmail.com",
    "name":"koteka exchain",
    "password":"asoi geboi"
}
```
{% endtab %}

{% tab title="Descriptions" %}
| field name | type | description |
| :--- | :--- | :--- |
| email | string | user email for register |
| name | string | user name for register |
| password | string | user password for auth |
{% endtab %}
{% endtabs %}



