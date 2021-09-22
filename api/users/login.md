# Login

{% api-method method="post" host="http://api-dev.mico.sg" path="/login" %}
{% api-method-summary %}
Users Login
{% endapi-method-summary %}

{% api-method-description %}
This endpoint allows you to get free cakes.
{% endapi-method-description %}

{% api-method-spec %}
{% api-method-request %}
{% api-method-body-parameters %}
{% api-method-parameter name="" type="object" required=true %}

{% endapi-method-parameter %}
{% endapi-method-body-parameters %}
{% endapi-method-request %}

{% api-method-response %}
{% api-method-response-example httpCode=200 %}
{% api-method-response-example-description %}
Cake successfully retrieved.
{% endapi-method-response-example-description %}

```javascript
{    "name": "{
    "responses": {
        "code": 400,
        "data": null,
        "isError": true,
        "message": "success"
    }
} 
```
{% endapi-method-response-example %}

{% api-method-response-example httpCode=400 %}
{% api-method-response-example-description %}
Could not find a cake matching this query.
{% endapi-method-response-example-description %}

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
{% endapi-method-response-example %}
{% endapi-method-response %}
{% endapi-method-spec %}
{% endapi-method %}

### Body Request

{% tabs %}
{% tab title="Json Body" %}
```javascript
{
    "email":"abdulx",
    "password":"assoi"
}
```
{% endtab %}

{% tab title="Description" %}
| fieldname | type | descriptipn |
| :--- | :--- | :--- |
| email | string | registered email |
| password | string | users password |
{% endtab %}
{% endtabs %}

